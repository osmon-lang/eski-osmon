use crate::error::VmError;
use crate::{frame::*, object::ObjectAddon, object_pool::ObjectPool, opcodes::*, value::Value};
use std::collections::HashMap;

macro_rules! for_c {
    ($v:ident = $v1:expr; $e:expr;$ex:expr, $b: block) => {
        let mut $v = $v1;
        while $e {
            $b
            $ex
        }
    };
}

///Machine that executes code
#[derive(Default)]
pub struct Machine {
    pub stack: Vec<CallFrame>,
    pub pool: ObjectPool,
    pub globals: HashMap<usize, Value>,
    pub labels: HashMap<usize, usize>,
}

impl Machine {
    pub fn new() -> Machine {
        Machine {
            stack: Vec::with_capacity(4096),
            pool: ObjectPool::new(),
            globals: HashMap::new(),
            labels: HashMap::new(),
        }
    }
    /// Get last frame in CallStack
    pub fn last_frame(&self) -> &CallFrame {
        self.stack.last().unwrap()
    }

    /// Get mutable reference to last frame in CallStack
    pub fn last_frame_mut(&mut self) -> &mut CallFrame {
        self.stack.last_mut().unwrap()
    }
    /// Get value for register
    pub fn get(&mut self, rnum: usize) -> Value {
        self.last_frame().get(rnum)
    }

    /// Set `this` value
    pub fn set_this(&mut self, v: Value) {
        self.last_frame_mut().stack[0] = v;
    }
    /// Set R(r) = v
    pub fn set(&mut self, r: usize, v: Value) {
        self.last_frame_mut().set(r, v);
    }
    /// Update instruction pointer
    pub fn dispatch(&mut self) {
        self.last_frame_mut().ip += 1;
    }
    /// Invoke callable object
    pub fn invoke(&mut self, callable: Value, args: Vec<Value>) -> Value {
        let id = match callable {
            Value::Object(id) => id,
            v => {
                panic!("Not callable {:?}", v);
            }
        };

        let obj = self.pool.get(id);
        self.stack.push(CallFrame::new());

        self.last_frame_mut().init_with_args(&args.as_slice());
        obj.call(self, args)
    }
    /// Goto
    pub fn branch(&mut self, idx: usize) {
        self.last_frame_mut().ip = idx;
    }
    /// Run instructions
    pub fn run_code(&mut self, code: Vec<Instruction>) -> Result<Value, VmError> {
        for_c!(i = 0;i < code.len();i += 1, {
            if let Instruction::Label(lbl_id) = code[i] {

                self.labels.insert(lbl_id, i);

            };
        });

        self.last_frame_mut().code = code;
        self.last_frame_mut().ip = 0;

        self.execute_op()
    }
    /// Execute all opcodes in current frame
    pub fn execute_op(&mut self) -> Result<Value, VmError> {
        let mut returns = false;
        let mut ret = Value::Null;
        let start = super::time::PreciseTime::now();

        while self.last_frame().ip < self.last_frame().code.len() {
            if returns {
                break;
            }

            let opcode = self.last_frame().code[self.last_frame().ip].clone();
            self.last_frame_mut().ip += 1;
            //println!("{:?}",self.last_frame().code[self.last_frame().ip]);
            match &opcode {
                Instruction::Label(_label_id) => {}

                Instruction::LoadArg(r1) => {
                    let value = self.get(*r1);
                    self.last_frame_mut().arg_stack.push(value);
                }

                Instruction::LoadBool(dest, boolean) => {
                    self.set(*dest, Value::Bool(*boolean));
                }

                Instruction::LoadInt(dest, int) => {
                    self.set(*dest, Value::Int(*int));
                }

                Instruction::LoadString(r1, ref string) => {
                    let string = string.to_string();
                    let object_id = self.pool.allocate(Box::new(string));
                    self.set(*r1, Value::Object(object_id));
                }

                Instruction::LoadDouble(dest, double) => {
                    self.set(*dest, Value::Double(*double));
                }

                Instruction::LoadLong(dest, long) => {
                    self.set(*dest, Value::Long(*long));
                }

                Instruction::LoadFloat(dest, float) => {
                    self.set(*dest, Value::Float(*float));
                }

                Instruction::Isa(dest, r1, r2) => {
                    let (v1, v2) = (self.get(*r1), self.get(*r2));
                    let n = v2.typename(self);
                    println!("{:?}", v2.typename(self));
                    let result = v1.isa(n, self);
                    self.set(*dest, Value::Bool(result));
                }

                Instruction::Add(dest, r1, r2) => {
                    let (v1, v2) = (self.get(*r1), self.get(*r2));

                    let result = match (v1, v2) {
                        (Value::Int(i), Value::Int(i2)) => Value::Int(i + i2),
                        (Value::Float(f), Value::Float(f2)) => Value::Float(f + f2),
                        (Value::Long(i), Value::Long(i2)) => Value::Long(i + i2),
                        (Value::Double(f), Value::Double(f2)) => Value::Double(f + f2),
                        (Value::Int(i), Value::Long(i2)) => Value::Long((i as i64) + i2),
                        (Value::Long(i), Value::Int(i2)) => Value::Long(i + (i2 as i64)),
                        (Value::Float(f), Value::Double(f2)) => Value::Double((f as f64) + f2),
                        (Value::Double(f), Value::Float(f2)) => Value::Double(f + (f2 as f64)),
                        (Value::Long(l), v) => Value::Long(l + v.to_long(self)),
                        (Value::Int(i), v) => Value::Int(i + v.to_int(self)),
                        (Value::Double(d), v) => Value::Double(d + v.to_double(self)),
                        (Value::Float(f), v) => Value::Float(f + v.to_float(self)),
                        (v, Value::Null) => v,
                        (Value::Null, v) => v,
                        v => panic!("{:?}", v),
                    };

                    self.set(*dest, result);
                }

                Instruction::Call(dest, r2, argc) => {
                    let args = {
                        let mut temp: Vec<Value> = vec![];
                        let this = self
                            .last_frame_mut()
                            .arg_stack
                            .pop()
                            .expect("Expected this value");

                        temp.push(this);

                        for _ in 0..*argc {
                            let v = self.last_frame_mut().arg_stack.pop();

                            match v {
                                None => temp.push(Value::Null), // if less arguments are passed then fill the holes with Null values
                                Some(v) => temp.push(v),
                            };
                        }

                        temp
                    };

                    let value = self.get(*r2);
                    let v = self.invoke(value, args);
                    self.stack.pop();
                    self.set(*dest, v);
                }
                Instruction::Sub(dest, r1, r2) => {
                    let (v1, v2) = (self.get(*r1), self.get(*r2));

                    let result = match (v1, v2) {
                        (Value::Int(i), Value::Int(i2)) => Value::Int(i - i2),
                        (Value::Float(f), Value::Float(f2)) => Value::Float(f - f2),
                        (Value::Long(i), Value::Long(i2)) => Value::Long(i - i2),
                        (Value::Double(f), Value::Double(f2)) => Value::Double(f - f2),
                        (Value::Int(i), Value::Long(i2)) => Value::Long((i as i64) - i2),
                        (Value::Long(i), Value::Int(i2)) => Value::Long(i - (i2 as i64)),
                        (Value::Float(f), Value::Double(f2)) => Value::Double((f as f64) - f2),
                        (Value::Double(f), Value::Float(f2)) => Value::Double(f - (f2 as f64)),
                        (Value::Long(l), v) => Value::Long(l - v.to_long(self)),
                        (Value::Int(i), v) => Value::Int(i - v.to_int(self)),
                        (Value::Double(d), v) => Value::Double(d - v.to_double(self)),
                        (Value::Float(f), v) => Value::Float(f - v.to_float(self)),
                        (v, Value::Null) => v,
                        (Value::Null, v) => v,
                        _ => unimplemented!(),
                    };

                    self.set(*dest, result);
                }

                Instruction::Div(dest, r1, r2) => {
                    let (v1, v2) = (self.get(*r1), self.get(*r2));

                    let result = match (v1, v2) {
                        (Value::Int(i), Value::Int(i2)) => Value::Int(i / i2),
                        (Value::Float(f), Value::Float(f2)) => Value::Float(f / f2),
                        (Value::Long(i), Value::Long(i2)) => Value::Long(i / i2),
                        (Value::Double(f), Value::Double(f2)) => Value::Double(f / f2),
                        (Value::Int(i), Value::Long(i2)) => Value::Long((i as i64) / i2),
                        (Value::Long(i), Value::Int(i2)) => Value::Long(i / (i2 as i64)),
                        (Value::Float(f), Value::Double(f2)) => Value::Double((f as f64) / f2),
                        (Value::Double(f), Value::Float(f2)) => Value::Double(f / (f2 as f64)),
                        (Value::Long(l), v) => Value::Long(l / v.to_long(self)),
                        (Value::Int(i), v) => Value::Int(i / v.to_int(self)),
                        (Value::Double(d), v) => Value::Double(d / v.to_double(self)),
                        (Value::Float(f), v) => Value::Float(f / v.to_float(self)),
                        (v, Value::Null) => v,
                        (Value::Null, v) => v,
                        _ => unimplemented!(),
                    };

                    self.set(*dest, result);
                }

                Instruction::Mul(dest, r1, r2) => {
                    let (v1, v2) = (self.get(*r1), self.get(*r2));

                    let result = match (v1, v2) {
                        (Value::Int(i), Value::Int(i2)) => Value::Int(i * i2),
                        (Value::Float(f), Value::Float(f2)) => Value::Float(f * f2),
                        (Value::Long(i), Value::Long(i2)) => Value::Long(i * i2),
                        (Value::Double(f), Value::Double(f2)) => Value::Double(f * f2),
                        (Value::Int(i), Value::Long(i2)) => Value::Long((i as i64) * i2),
                        (Value::Long(i), Value::Int(i2)) => Value::Long(i * (i2 as i64)),
                        (Value::Float(f), Value::Double(f2)) => Value::Double((f as f64) * f2),
                        (Value::Double(f), Value::Float(f2)) => Value::Double(f * (f2 as f64)),
                        (Value::Long(l), v) => Value::Long(l * v.to_long(self)),
                        (Value::Int(i), v) => Value::Int(i * v.to_int(self)),
                        (Value::Double(d), v) => Value::Double(d * v.to_double(self)),
                        (Value::Float(f), v) => Value::Float(f * v.to_float(self)),
                        (v, Value::Null) => v,
                        (Value::Null, v) => v,
                        v => panic!("Cannot mul {:?}", v),
                    };

                    self.set(*dest, result);
                }

                Instruction::LoadConst(r1, idx) => {
                    self.set(*r1, Value::Object(*idx));
                }

                Instruction::Not(r1, r2) => {
                    let v = self.get(*r2);
                    let result = Value::Bool(v.not(self));
                    self.set(*r1, result);
                }

                Instruction::Gt(dest, r1, r2) => {
                    let (v1, v2) = (self.get(*r1), self.get(*r2));
                    let result = match (v1, v2) {
                        (Value::Int(i), Value::Int(i2)) => Value::Bool(i > i2),
                        (Value::Long(i), Value::Long(i2)) => Value::Bool(i > i2),
                        (Value::Float(f), Value::Float(f2)) => Value::Bool(f > f2),
                        (Value::Double(f), Value::Double(f2)) => Value::Bool(f > f2),
                        (Value::Int(i), Value::Long(i2)) => Value::Bool((i as i64) > i2),
                        (Value::Long(i), Value::Int(i2)) => Value::Bool(i > (i2 as i64)),
                        (Value::Float(f), Value::Double(f2)) => Value::Bool((f as f64) > f2),
                        (Value::Double(f), Value::Float(f2)) => Value::Bool(f > (f2 as f64)),
                        (Value::Long(l), v) => Value::Bool(l > v.to_long(self)),
                        (Value::Int(i), v) => Value::Bool(i > v.to_int(self)),
                        (Value::Double(d), v) => Value::Bool(d > v.to_double(self)),
                        (Value::Float(f), v) => Value::Bool(f > v.to_float(self)),
                        (v, Value::Null) => v,
                        (Value::Null, v) => v,
                        v => panic!("{:?}", v),
                    };

                    self.set(*dest, result);
                }
                Instruction::Ge(dest, r1, r2) => {
                    let (v1, v2) = (self.get(*r1), self.get(*r2));
                    let result = match (v1, v2) {
                        (Value::Int(i), Value::Int(i2)) => Value::Bool(i >= i2),
                        (Value::Long(i), Value::Long(i2)) => Value::Bool(i >= i2),
                        (Value::Float(f), Value::Float(f2)) => Value::Bool(f >= f2),
                        (Value::Double(f), Value::Double(f2)) => Value::Bool(f >= f2),
                        (Value::Int(i), Value::Long(i2)) => Value::Bool((i as i64) >= i2),
                        (Value::Long(i), Value::Int(i2)) => Value::Bool(i >= (i2 as i64)),
                        (Value::Float(f), Value::Double(f2)) => Value::Bool((f as f64) >= f2),
                        (Value::Double(f), Value::Float(f2)) => Value::Bool(f >= (f2 as f64)),
                        (Value::Long(l), v) => Value::Bool(l >= v.to_long(self)),
                        (Value::Int(i), v) => Value::Bool(i >= v.to_int(self)),
                        (Value::Double(d), v) => Value::Bool(d >= v.to_double(self)),
                        (Value::Float(f), v) => Value::Bool(f >= v.to_float(self)),
                        (v, Value::Null) => v,
                        (Value::Null, v) => v,
                        _ => unimplemented!(),
                    };

                    self.set(*dest, result);
                }

                Instruction::Le(dest, r1, r2) => {
                    let (v1, v2) = (self.get(*r1), self.get(*r2));
                    let result = match (v1, v2) {
                        (Value::Int(i), Value::Int(i2)) => Value::Bool(i <= i2),
                        (Value::Long(i), Value::Long(i2)) => Value::Bool(i <= i2),
                        (Value::Float(f), Value::Float(f2)) => Value::Bool(f <= f2),
                        (Value::Double(f), Value::Double(f2)) => Value::Bool(f <= f2),
                        (Value::Int(i), Value::Long(i2)) => Value::Bool((i as i64) <= i2),
                        (Value::Long(i), Value::Int(i2)) => Value::Bool(i <= (i2 as i64)),
                        (Value::Float(f), Value::Double(f2)) => Value::Bool((f as f64) <= f2),
                        (Value::Double(f), Value::Float(f2)) => Value::Bool(f <= (f2 as f64)),
                        (Value::Long(l), v) => Value::Bool(l <= v.to_long(self)),
                        (Value::Int(i), v) => Value::Bool(i <= v.to_int(self)),
                        (Value::Double(d), v) => Value::Bool(d <= v.to_double(self)),
                        (Value::Float(f), v) => Value::Bool(f <= v.to_float(self)),
                        (v, Value::Null) => v,
                        (Value::Null, v) => v,
                        v => panic!("Unimplemented {:?}", v),
                    };

                    self.set(*dest, result);
                }

                Instruction::Lt(dest, r1, r2) => {
                    let (v1, v2) = (self.get(*r1), self.get(*r2));
                    let result = match (v1, v2) {
                        (Value::Int(i), Value::Int(i2)) => Value::Bool(i < i2),
                        (Value::Long(i), Value::Long(i2)) => Value::Bool(i < i2),
                        (Value::Float(f), Value::Float(f2)) => Value::Bool(f < f2),
                        (Value::Double(f), Value::Double(f2)) => Value::Bool(f < f2),
                        (Value::Int(i), Value::Long(i2)) => Value::Bool((i as i64) < i2),
                        (Value::Long(i), Value::Int(i2)) => Value::Bool(i < (i2 as i64)),
                        (Value::Float(f), Value::Double(f2)) => Value::Bool((f as f64) < f2),
                        (Value::Double(f), Value::Float(f2)) => Value::Bool(f < (f2 as f64)),
                        (Value::Long(l), v) => Value::Bool(l < v.to_long(self)),
                        (Value::Int(i), v) => Value::Bool(i < v.to_int(self)),
                        (Value::Double(d), v) => Value::Bool(d < v.to_double(self)),
                        (Value::Float(f), v) => Value::Bool(f < v.to_float(self)),
                        (v, Value::Null) => v,
                        (Value::Null, v) => v,
                        (v, v1) => panic!("{:?} < {:?}", v.to_String(self), v1.to_String(self)),
                    };

                    self.set(*dest, result);
                }
                Instruction::BitAnd(r3, r1, r2) => {
                    let (v1, v2) = (self.get(*r1), self.get(*r2));

                    let result = match (v1, v2) {
                        (Value::Long(l), Value::Long(l1)) => Value::Long(l & l1),
                        (Value::Int(i), Value::Int(i2)) => Value::Int(i & i2),
                        v => panic!("BitAnd cannot be aplied to {:?}", v),
                    };
                    self.set(*r3, result);
                }
                Instruction::BitOr(r3, r1, r2) => {
                    let (v1, v2) = (self.get(*r1), self.get(*r2));

                    let result = match (v1, v2) {
                        (Value::Long(l), Value::Long(l1)) => Value::Long(l | l1),
                        (Value::Int(i), Value::Int(i2)) => Value::Int(i | i2),
                        v => panic!("BitOr cannot be aplied to {:?}", v),
                    };
                    self.set(*r3, result);
                }
                Instruction::BitXor(r3, r1, r2) => {
                    let (v1, v2) = (self.get(*r1), self.get(*r2));

                    let result = match (v1, v2) {
                        (Value::Long(l), Value::Long(l1)) => Value::Long(l ^ l1),
                        (Value::Int(i), Value::Int(i2)) => Value::Int(i ^ i2),
                        v => panic!("BitOr cannot be aplied to {:?}", v),
                    };
                    self.set(*r3, result);
                }
                Instruction::Shl(r3, r1, r2) => {
                    let (v1, v2) = (self.get(*r1), self.get(*r2));

                    let result = match (v1, v2) {
                        (Value::Long(l), Value::Long(l1)) => Value::Long(l << l1),
                        (Value::Int(i), Value::Int(i2)) => Value::Int(i << i2),
                        v => panic!("BitOr cannot be aplied to {:?}", v),
                    };
                    self.set(*r3, result);
                }
                Instruction::Shr(r3, r1, r2) => {
                    let (v1, v2) = (self.get(*r1), self.get(*r2));

                    let result = match (v1, v2) {
                        (Value::Long(l), Value::Long(l1)) => Value::Long(l >> l1),
                        (Value::Int(i), Value::Int(i2)) => Value::Int(i >> i2),
                        v => panic!("BitOr cannot be aplied to {:?}", v),
                    };
                    self.set(*r3, result);
                }
                Instruction::And(r3, r1, r2) => {
                    let (v1, v2) = (self.get(*r1), self.get(*r2));

                    let result = match (v1, v2) {
                        (Value::Bool(b), Value::Bool(b2)) => Value::Bool(b && b2),
                        v => panic!("And cannot be aplied to {:?}", v),
                    };

                    self.set(*r3, result);
                }
                Instruction::Or(r3, r1, r2) => {
                    let (v1, v2) = (self.get(*r1), self.get(*r2));

                    let result = match (v1, v2) {
                        (Value::Bool(b), Value::Bool(b2)) => Value::Bool(b || b2),
                        v => panic!("Or cannot be aplied to {:?}", v),
                    };

                    self.set(*r3, result);
                }
                Instruction::Eq(r3, r1, r2) => {
                    let (v1, v2) = (self.get(*r1), self.get(*r2));
                    let result = match (v1, v2) {
                        (Value::Int(i), Value::Int(i2)) => Value::Bool(i == i2),
                        (Value::Long(i), Value::Long(i2)) => Value::Bool(i == i2),
                        (Value::Float(f), Value::Float(f2)) => Value::Bool(f == f2),
                        (Value::Double(f), Value::Double(f2)) => Value::Bool(f == f2),
                        (Value::Int(i), Value::Long(i2)) => Value::Bool((i as i64) == i2),
                        (Value::Long(i), Value::Int(i2)) => Value::Bool(i == (i2 as i64)),
                        (Value::Float(f), Value::Double(f2)) => Value::Bool((f as f64) == f2),
                        (Value::Double(f), Value::Float(f2)) => Value::Bool(f == (f2 as f64)),
                        (Value::Long(l), v) => Value::Bool(l == v.to_long(self)),
                        (Value::Int(i), v) => Value::Bool(i == v.to_int(self)),
                        (Value::Double(d), v) => Value::Bool(d == v.to_double(self)),
                        (Value::Float(f), v) => Value::Bool(f == v.to_float(self)),
                        (_v, Value::Null) => Value::Bool(false),
                        (Value::Null, _v) => Value::Bool(false),
                        _ => unimplemented!(),
                    };

                    self.set(*r3, result);
                }

                Instruction::Neq(r3, r1, r2) => {
                    let (v1, v2) = (self.get(*r1), self.get(*r2));
                    let result = match (v1, v2) {
                        (Value::Int(i), Value::Int(i2)) => Value::Bool(i != i2),
                        (Value::Long(i), Value::Long(i2)) => Value::Bool(i != i2),
                        (Value::Float(f), Value::Float(f2)) => Value::Bool(f != f2),
                        (Value::Double(f), Value::Double(f2)) => Value::Bool(f != f2),
                        (Value::Int(i), Value::Long(i2)) => Value::Bool((i as i64) != i2),
                        (Value::Long(i), Value::Int(i2)) => Value::Bool(i == (i2 as i64)),
                        (Value::Float(f), Value::Double(f2)) => Value::Bool((f as f64) != f2),
                        (Value::Double(f), Value::Float(f2)) => Value::Bool(f != (f2 as f64)),
                        (Value::Long(l), v) => Value::Bool(l != v.to_long(self)),
                        (Value::Int(i), v) => Value::Bool(i != v.to_int(self)),
                        (Value::Double(d), v) => Value::Bool(d != v.to_double(self)),
                        (Value::Float(f), v) => Value::Bool(f != v.to_float(self)),
                        (_v, Value::Null) => Value::Bool(false),
                        (Value::Null, _v) => Value::Bool(false),
                        _ => unimplemented!(),
                    };

                    self.set(*r3, result);
                }

                Instruction::Goto(lbl_id) => {
                    if self.labels.contains_key(lbl_id) {
                        let idx = &self.labels[lbl_id];
                        self.branch(*idx + 1);
                    } else {
                        return Err(VmError::LabelNotFound(*lbl_id));
                    }
                }

                Instruction::GotoF(r1, lbl_id) => match self.get(*r1) {
                    Value::Bool(b) => {
                        if !b {
                            if self.labels.contains_key(lbl_id) {
                                let idx = &self.labels[lbl_id];
                                self.branch(*idx + 1);
                            } else {
                                return Err(VmError::LabelNotFound(*lbl_id));
                            }
                        }
                    }

                    _v => return Err(VmError::RuntimeError("GotoF exptected Bool value".into())),
                },

                Instruction::Jump(idx) => {
                    self.branch(*idx);
                }

                Instruction::LoadGlobal(r1, index) => {
                    if self.globals.contains_key(index) {
                        let value = &self.globals[index];
                        self.set(*r1, *value);
                    } else {
                        return Err(VmError::GlobalNotFound(*index));
                    }
                }

                Instruction::StoreGlobal(r1, index) => {
                    let value = self.get(*r1);
                    self.globals.insert(*index, value);
                }

                Instruction::JumpF(r1, idx) => {
                    let v = self.get(*r1);
                    if let Value::Bool(b) = v {
                        if !b {
                            self.branch(*idx);
                        }
                    } else {
                        return Err(VmError::RuntimeError(
                            "Expected Bool value; Op JumpF".into(),
                        ));
                    }
                }

                Instruction::Move(r1, r2) => {
                    let v = self.get(*r2);

                    self.last_frame_mut().stack[*r1] = v;
                }

                Instruction::Ret(idx) => {
                    ret = self.get(*idx);
                    returns = true;
                }

                Instruction::Ret0 => {
                    returns = true;
                }

                Instruction::LoadAt(r1, r2, r3) => {
                    let v2 = self.get(*r2);
                    let v3 = self.get(*r3);

                    if let Value::Object(obj_id) = v2 {
                        let obj = self.pool.get(obj_id);
                        let this = self.get(*r2);
                        obj.load_at(self, vec![this, v3], *r1);
                    } else {
                        return Err(VmError::Expected(
                            "Value::Object".into(),
                            format!("{:?}", v2),
                        ));
                    }
                }

                Instruction::StoreAt(r1, r2, r3) => {
                    let value = self.get(*r1);
                    let target = self.get(*r2);
                    let key = self.get(*r3);
                    if let Value::Object(obj_id) = &target {
                        let obj = self.pool.get(*obj_id);
                        obj.store_at(self, vec![target, key, value], 0);
                    } else {
                        return Err(VmError::Expected(
                            "Value::Object".into(),
                            format!("{:?}", &target),
                        ));
                    }
                }

                v => panic!("{:?}", v),
            }
        }
        let end = super::time::PreciseTime::now();

        let _result = start.to(end).num_milliseconds();

        Ok(ret)
    }
}
