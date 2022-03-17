use crate::{machine::Machine, object::Object, opcodes::*, value::Value};
use std::any::Any;

#[derive(Debug)]
pub enum Function {
    Virtual(VirtualFunction),
    Native(NativeFunction),
}

impl Clone for Function {
    fn clone(&self) -> Function {
        match self {
            Function::Virtual(vf) => Function::Virtual(vf.clone()),
            Function::Native(_) => panic!("Cannot clone native func"),
        }
    }
}

impl crate::object::ObjectAddon for Function {
    fn typename(&self, _: &mut Machine) -> String {
        return "Func".into();
    }

    fn to_String(&self, _m: &mut Machine) -> String {
        String::from("function")
    }

    fn as_function(&self) -> &Function {
        self
    }
}

impl Object for Function {
    /// Call object
    fn call(&self, m: &mut Machine, args: Vec<Value>) -> Value {
        match self {
            Function::Virtual(ref vf) => {
                //println!("{:?}",args);
                let func = vf.clone();
                //println!("{:?}",args[0].to_String(m));
                m.last_frame_mut().stack[0] = args[0];
                for i in 0..args.len() {
                    m.last_frame_mut().stack[i] = args[i];
                }
                let code = func.code;
                let v = m.run_code(code);
                match v {
                    Ok(v) => return v,
                    Err(e) => {
                        eprintln!("{}", e);
                        panic!("");
                    }
                }
            }

            Function::Native(nv) => nv.0(m, args),
        }
    }

    fn load_at(&self, m: &mut Machine, _args: Vec<Value>, dest: usize) {
        let _this = _args[0];
        let val = if let Value::Object(id) = &_args[1] {
            m.pool.get(*id)
        } else {
            panic!("Exptected object")
        };

        let fname: &str = &val.to_String(m);

        match fname {
            "disassemble" => {
                let code = if let Function::Virtual(vf) = self {
                    vf.code.toString()
                } else {
                    "<native function>".to_string()
                };
                let obj = m.pool.allocate(Box::new(code));
                let code = vec![Instruction::LoadConst(1, obj), Instruction::Ret(1)];
                let func = Function::from(code);
                let obj = m.pool.allocate(Box::new(func));
                m.set(dest, Value::Object(obj));
            }
            f => panic!("Unknown field `{}`", f),
        }
    }

    fn as_any(&self) -> &dyn Any {
        self as &dyn Any
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self as &mut dyn Any
    }

    /// Get Object Id's(Used for GC) W.I.P
    fn get_children(&self) -> Vec<usize> {
        vec![]
    }
}

#[derive(Clone, Debug)]
pub struct VirtualFunction {
    pub code: Vec<Instruction>,
    pub argc: usize,
}

impl Function {
    pub fn from_instructions(code: Vec<Instruction>, args: usize) -> Function {
        Function::Virtual(VirtualFunction { code, argc: args })
    }

    pub fn from_native(f: Box<dyn Fn(&mut Machine, Vec<Value>) -> Value + Send>) -> Function {
        Function::Native(NativeFunction(f))
    }
}

impl From<Vec<Instruction>> for Function {
    fn from(f: Vec<Instruction>) -> Function {
        Function::Virtual(VirtualFunction { code: f, argc: 0 })
    }
}

pub struct NativeFunction(pub Box<dyn Fn(&mut Machine, Vec<Value>) -> Value + Send>);

impl NativeFunction {
    pub fn invoke(&self, m: &mut Machine, args: Vec<Value>) -> Value {
        self.0(m, args)
    }
}

use std::fmt;

impl fmt::Debug for NativeFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<native function>")
    }
}
