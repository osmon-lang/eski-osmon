use crate::{builtins::*, class::Class, std_library::*};

use float_duration::TimePoint;
use std::time::Instant;

use std::collections::HashMap;

use crate::{
    code::FunctionBuilder,
    parser::{Expr, FnDef, Global, Op, Stmt},
};
use bulut::{
    function::Function,
    machine::Machine,
    object::ObjectAddon,
    opcodes::{DebugCode, Instruction},
    value::Value,
};

pub struct Compiler<'a> {
    pub machine: &'a mut Machine,
    pub builder: FunctionBuilder,
    pub gp: usize,
    pub globals: HashMap<String, usize>,
    pub debug: bool,
}

impl<'a> Compiler<'a> {
    pub fn new(m: &'a mut Machine, argc: usize, debug: bool) -> Compiler<'a> {
        let mut compiler = Compiler {
            machine: m,
            builder: FunctionBuilder::new(argc),
            globals: HashMap::new(),
            gp: 0,
            debug,
        };
        compiler.register_builtins();
        compiler
    }

    pub fn register_builtins(&mut self) {
        self.gp += 1;

        let id = self
            .machine
            .pool
            .allocate(Box::new(Function::from_native(Box::new(print))));
        self.globals.insert("yoz".to_string(), self.gp);
        self.machine.globals.insert(self.gp, Value::Object(id));
        self.gp += 1;
        let id = self
            .machine
            .pool
            .allocate(Box::new(Function::from_native(Box::new(readln))));
        self.globals.insert("oqi".to_string(), self.gp);
        self.machine.globals.insert(self.gp, Value::Object(id));

        self.gp += 1;
        let id = self
            .machine
            .pool
            .allocate(Box::new(Function::from_native(Box::new(new_array))));
        self.globals.insert("__new_array__".to_string(), self.gp);
        self.machine.globals.insert(self.gp, Value::Object(id));
        self.gp += 1;
        let id = self
            .machine
            .pool
            .allocate(Box::new(Function::from_native(Box::new(concat))));
        self.globals.insert("tahrirla".to_owned(), self.gp);
        self.machine.globals.insert(self.gp, Value::Object(id));
        self.gp += 1;
        let class = system_class(self.machine);
        let id = self.machine.pool.allocate(Box::new(class));
        self.globals.insert("Tizim".to_owned(), self.gp);
        self.machine.globals.insert(self.gp, Value::Object(id));
        self.gp += 1;
        self.globals.insert("__unary_minus__".to_owned(), self.gp);
        let f = unary_minus(self.machine);
        self.machine.globals.insert(self.gp, f);
        self.gp += 1;
        let class = int_class();
        let id = self.machine.pool.allocate(Box::new(class));
        self.globals.insert("Int".to_owned(), self.gp);
        self.machine.globals.insert(self.gp, Value::Object(id));
        self.gp += 1;
        let class = float_class();
        let id = self.machine.pool.allocate(Box::new(class));
        self.globals.insert("Float".to_owned(), self.gp);
        self.machine.globals.insert(self.gp, Value::Object(id));
        self.gp += 1;
        let class = str_class();
        let id = self.machine.pool.allocate(Box::new(class));
        self.globals.insert("Str".to_owned(), self.gp);
        self.machine.globals.insert(self.gp, Value::Object(id));
    }

    pub fn compile(&mut self, globals: Vec<Global>) -> Value {
        for global in globals.iter() {
            if let Global::ClassDefinition(ref class) = &global {
                let name = if let Expr::Identifier(ref n) = &*class.name {
                    n.to_string()
                } else {
                    "<unknown>".to_string()
                };
                self.machine.globals.insert(self.gp, Value::Int(255));
                self.globals.insert(name, self.gp);
            };

            if let Global::FnDefenition(ref fun) = &global {
                let name = if let Expr::Identifier(ref n) = &*fun.name {
                    n.to_string()
                } else {
                    "<unknown>".to_string()
                };
                self.machine.globals.insert(self.gp, Value::Int(0));

                self.globals.insert(name.clone(), self.gp);
            }
            self.gp += 1;
        }

        for global in globals.iter() {
            if let Global::ClassDefinition(ref classdef) = global {
                let mut class = Class::new();

                let name = if let Expr::Identifier(ref n) = &*classdef.name.clone() {
                    n.to_string()
                } else {
                    "<undefined>".to_string()
                };

                class.name = name.clone();

                for fun in classdef.methods.iter() {
                    let fun: FnDef = fun.clone();
                    let name = if let Expr::Identifier(ref n) = &*fun.name.clone() {
                        n.to_string()
                    } else {
                        "<undefined>".to_string()
                    };
                    let builder = FunctionBuilder::new(fun.params.len());
                    self.builder = builder;
                    for param in &fun.params {
                        let reg = self.builder.register_first_temp_available();
                        self.builder.new_local(param.to_string(), reg);
                    }
                    self.translate_stmt(*fun.body);
                    let code = self.builder.get_insts();
                    let func = Function::from_instructions(code, fun.params.len());
                    let func = self.machine.pool.allocate(Box::new(func));
                    unsafe { (&mut *class.fields.get()).insert(name, Value::Object(func)) };
                }
                for (name, expr) in classdef.vars.iter() {
                    let name = name.to_string();
                    if expr.is_some() {
                        match &expr.clone().unwrap() {
                            Expr::IntConst(int) => {
                                unsafe {
                                    (&mut *class.fields.get()).insert(name, Value::Int(*int as i32))
                                };
                            }
                            Expr::FloatConst(float) => {
                                unsafe {
                                    (&mut *class.fields.get())
                                        .insert(name, Value::Float(*float as f32))
                                };
                            }
                            Expr::StringConst(str) => {
                                let obj = Value::Object(
                                    self.machine.pool.allocate(Box::new(str.to_string())),
                                );
                                unsafe { (&mut *class.fields.get()).insert(name, obj) };
                            }
                            _ => unsafe {
                                (&mut *class.fields.get()).insert(name, Value::Null);
                            },
                        }
                    } else {
                        unsafe {
                            (&mut *class.fields.get()).insert(name, Value::Null);
                        };
                    }
                }

                let ptr = self.globals.get(&name).unwrap().clone();

                let cls = self.machine.pool.allocate(Box::new(class));
                self.machine.globals.insert(ptr, Value::Object(cls));
                self.globals.insert(name, ptr);
            }

            if let Global::FnDefenition(ref fun) = global {
                let name = if let Expr::Identifier(ref n) = &*fun.name.clone() {
                    n.to_string()
                } else {
                    "asosiy".to_string()
                };

                let builder = FunctionBuilder::new(fun.params.len());
                self.builder = builder;
                for param in &fun.params {
                    let reg = self.builder.register_first_temp_available();

                    self.builder.new_local(param.to_string(), reg);
                }
                self.translate_stmt(*fun.clone().body);

                let code = self.builder.get_insts();

                if self.debug {
                    println!("function `{}` code: ", name);
                    println!("{}", code.toString());
                }

                let function = Function::from_instructions(code, fun.params.len());
                let func = self.machine.pool.allocate(Box::new(function));
                let ptr = self.globals.get(&name).unwrap().clone();
                self.machine.globals.insert(ptr, Value::Object(func));
                self.globals.insert(name, ptr);
            }
        }

        let main = self
            .globals
            .get("asosiy")
            .expect("asosiy topilmadi")
            .clone();
        let main = self.machine.globals.get(&main).expect("asosiy topilmadi");
        let start = Instant::now();
        let ret = self.machine.invoke(*main, vec![Value::Null]);
        let end = Instant::now();
        println!(
            "Natija: {} ({} ichida)",
            ret.to_String(self.machine),
            end.float_duration_since(start).unwrap()
        );
        ret
    }

    pub fn translate_stmt(&mut self, s: Stmt) {
        match s {
            Stmt::If(condition, then) => {
                let then = *then;
                let label_false = self.builder.new_label();

                self.translate_expr(*condition);
                let reg = self.builder.register_pop();
                self.builder.push_op(Instruction::GotoF(reg, label_false));
                self.translate_stmt(then);
                self.builder.label_here(label_false);
            }

            Stmt::IfElse(condition, if_true, if_false) => {
                let label_false = self.builder.new_label();

                self.translate_expr(*condition);
                let reg = self.builder.register_pop();
                self.builder.push_op(Instruction::GotoF(reg, label_false));
                self.translate_stmt(*if_true);
                self.builder.label_here(label_false);
                self.translate_stmt(*if_false);
            }

            Stmt::For(value, condition, expr, block) => {
                let compare = self.builder.new_label();
                let end = self.builder.new_label();

                self.translate_stmt(*value);
                self.builder.label_here(compare);

                self.translate_expr(*condition);
                let reg = self.builder.register_pop();
                self.builder.push_op(Instruction::GotoF(reg, end));
                self.translate_stmt(*block);
                self.translate_expr(*expr);
                self.builder.push_op(Instruction::Goto(compare));
                self.builder.label_here(end);
            }

            Stmt::While(condition, block) => {
                let compare = self.builder.new_label();
                let end = self.builder.new_label();

                self.builder.label_here(compare);

                self.translate_expr(*condition);
                let reg = self.builder.register_pop();
                self.builder.push_op(Instruction::GotoF(reg, end));
                self.translate_stmt(*block);
                self.builder.push_op(Instruction::Goto(compare));
                self.builder.label_here(end);
            }

            Stmt::Var(ref name, ref expr) => {
                let name = name.to_string();
                let expr = expr.clone();

                if expr.is_some() {
                    self.translate_expr(*expr.unwrap().clone());
                    let r = self.builder.register_pop();
                    self.builder.new_local(name, r);
                } else {
                    let r = self.builder.register_first_temp_available();
                    self.builder.new_local(name, r);
                }
            }
            Stmt::Return => {
                self.builder.push_op(Instruction::Ret0);
            }
            Stmt::ReturnWithVal(val) => {
                self.translate_expr(*val);
                let r = self.builder.register_pop();
                self.builder.push_op(Instruction::Ret(r));
            }

            Stmt::Block(body) => {
                for stmt in body.iter() {
                    self.translate_stmt(stmt.clone());
                }
            }
            Stmt::Expr(expr) => {
                self.translate_expr(*expr.clone());
            }
            v => panic!("{:?}", v),
        }
    }

    pub fn translate_expr(&mut self, expr: Expr) {
        match expr {
            Expr::IntConst(int) => {
                self.builder.long_const(int);
            }
            Expr::FloatConst(float) => {
                self.builder.double_const(float);
            }

            Expr::This => {
                let r = self.builder.register_push_temp();
                self.builder.push_op(Instruction::Move(r, 0));
            }

            Expr::FnCall(ref fname, ref args) => {
                let mut args = args.clone();
                args.reverse();
                for arg in args.iter() {
                    self.translate_expr(arg.clone());
                    let r = self.builder.register_pop();
                    self.builder.push_op(Instruction::LoadArg(r));
                }

                let fptr = if !self.globals.contains_key(fname) {
                    let r = self.builder.get_local(fname);
                    r
                } else {
                    let idx = self
                        .globals
                        .get(fname)
                        .expect(&format!("Function not found `{}`", fname));
                    let register = self.builder.register_push_temp();
                    self.builder
                        .push_op(Instruction::LoadGlobal(register, *idx));
                    register
                };
                let mut dest = fptr;
                let mut _dest_is_temp = self.builder.register_is_temp(dest);
                if !_dest_is_temp {
                    dest = self.builder.register_push_temp();
                    _dest_is_temp = true;
                }
                self.builder.push_op(Instruction::LoadArg(fptr));
                self.builder
                    .push_op(Instruction::Call(dest, fptr, args.len()));
            }

            Expr::New(name, args) => {
                let mut args = args.clone();
                args.reverse();
                for arg in args.iter() {
                    self.translate_expr(arg.clone());
                    let r = self.builder.register_pop();
                    self.builder.push_op(Instruction::LoadArg(r));
                }

                let dest = self.builder.register_push_temp();
                let fptr = if !self.globals.contains_key(&name) {
                    let r = self.builder.get_local(&name);
                    r
                } else {
                    let idx = self
                        .globals
                        .get(&name)
                        .expect(&format!("Function not found `{}`", name));
                    let register = self.builder.register_first_temp_available();
                    self.builder
                        .push_op(Instruction::LoadGlobal(register, *idx));
                    register
                };

                self.builder.push_op(Instruction::LoadArg(fptr));
                self.builder
                    .push_op(Instruction::Call(dest, fptr, args.len()));
            }

            Expr::Array(arr_expr) => {
                for expr in arr_expr.iter() {
                    self.translate_expr(expr.clone());
                    let r = self.builder.register_pop();
                    self.builder.push_op(Instruction::LoadArg(r));
                }

                let id = self.globals.get("__new_array__").unwrap();
                let reg = self.builder.register_push_temp();
                self.builder.push_op(Instruction::LoadGlobal(reg, *id));
                self.builder.push_op(Instruction::LoadArg(reg));
                let dest = self.builder.register_push_temp();
                self.builder
                    .push_op(Instruction::Call(dest, reg, arr_expr.len()));
            }

            Expr::Op(op, e1, e2) => {
                self.translate_operation(op, e1, e2);
            }

            Expr::StringConst(ref s) => {
                let r = self.builder.register_push_temp();
                self.builder
                    .push_op(Instruction::LoadString(r, s.to_string()));
            }

            Expr::Identifier(ref name) => {
                if !self.globals.contains_key(name) {
                    let r = self.builder.get_local(name);
                    let r2 = self.builder.register_push_temp();
                    self.builder.push_op(Instruction::Move(r2, r));
                } else {
                    let idx = self.globals.get(name).unwrap();
                    let register = self.builder.register_push_temp();
                    self.builder
                        .push_op(Instruction::LoadGlobal(register, *idx));
                }
            }

            Expr::Assignment(e1, e2) => {
                let e1 = *e1;
                let e2 = *e2;

                if let Expr::Identifier(ref name) = e1 {
                    self.translate_expr(e2.clone());
                    if self.globals.contains_key(name) {
                        let id = self.globals.get(name).unwrap();
                        let r = self.builder.register_pop();
                        self.builder.push_op(Instruction::StoreGlobal(r, *id));
                    } else {
                        let r1 = self.builder.get_local(name);
                        let r2 = self.builder.register_pop();
                        self.builder.push_op(Instruction::Move(r1, r2));
                    }
                }
                if let Expr::Op(Op::Access, this, fname) = e1 {
                    let r2 = self.builder.register_push_temp();

                    if let Expr::Identifier(n) = *fname {
                        self.builder.push_op(Instruction::LoadString(r2, n));
                    } else {
                        panic!("");
                    };

                    self.translate_expr(e2.clone());
                    let value = self.builder.register_pop();
                    self.builder.register_push_temp();
                    self.translate_expr(*this);
                    let this = self.builder.register_pop();
                    self.builder.push_op(Instruction::StoreAt(value, this, r2));
                }
            }
            Expr::False => {
                let reg = self.builder.register_push_temp();
                self.builder.push_op(Instruction::LoadBool(reg, false));
            }
            Expr::True => {
                let reg = self.builder.register_push_temp();
                self.builder.push_op(Instruction::LoadBool(reg, true));
            }
            Expr::Index(name, idx) => {
                let target = if self.globals.contains_key(&name) {
                    let gp = self.globals.get(&name).unwrap();
                    let dest = self.builder.register_push_temp();
                    self.builder.push_op(Instruction::LoadGlobal(dest, *gp));
                    dest
                } else {
                    let r = self.builder.get_local(&name);
                    let r2 = self.builder.register_push_temp();
                    self.builder.push_op(Instruction::Move(r2, r));
                    r2
                };
                self.translate_expr(*idx);
                let reg = self.builder.register_pop();

                let dest = self.builder.register_push_temp();
                self.builder.push_op(Instruction::LoadAt(dest, target, reg));
            }
            Expr::Unit => {
                let _r = self.builder.register_push_temp();
            }
            v => panic!("Unimplemented {:?}", v),
        }
    }

    pub fn translate_operation(&mut self, op: Op, e1: Box<Expr>, e2: Box<Expr>) {
        if op == Op::Access {
            match (*e1, *e2) {
                (this, Expr::Identifier(field)) => {
                    let r2 = self.builder.register_push_temp();
                    self.translate_expr(this);
                    let r1 = self.builder.register_pop();
                    self.builder.push_op(Instruction::LoadString(r2, field));
                    let r3 = self.builder.register_push_temp();
                    self.builder.push_op(Instruction::LoadAt(r3, r1, r2));
                    self.builder.register_clear(r1);
                    self.builder.register_clear(r2);
                }
                (this, Expr::FnCall(fname, args)) => {
                    let mut args = args.clone();
                    args.reverse();
                    for arg in args.iter() {
                        self.translate_expr(arg.clone());
                        let r = self.builder.register_pop();

                        self.builder.push_op(Instruction::LoadArg(r));
                    }
                    self.translate_expr(this);
                    let r1 = self.builder.register_pop();
                    let mut r2 = self.builder.register_push_temp();
                    if self.builder.register_is_temp(r2) {
                        r2 = self.builder.register_push_temp();
                    }
                    let r3 = self.builder.register_push_temp();
                    self.builder.push_op(Instruction::LoadArg(r1));

                    self.builder.push_op(Instruction::LoadString(r2, fname));

                    self.builder.push_op(Instruction::LoadAt(r3, r1, r2));
                    //self.builder.register_pop();
                    let mut dest = self.builder.register_push_temp();
                    let mut _dest_is_temp = self.builder.register_is_temp(dest);
                    if !_dest_is_temp {
                        dest = self.builder.register_push_temp();
                        _dest_is_temp = true;
                    }
                    self.builder
                        .push_op(Instruction::Call(dest, r3, args.len()));
                    self.builder.register_clear(r3);
                    self.builder.register_clear(r1);
                    self.builder.register_clear(r2);
                }
                v => panic!("Unimplemented {:?}", v),
            }
        } else {
            if op == Op::Not {
                let e1 = *e1;

                self.translate_expr(e1);
                let r = self.builder.register_pop();
                let r2 = self.builder.register_push_temp();

                self.builder.push_op(Instruction::Not(r2, r));
                return;
            }

            let e1 = *e1;
            let e2 = *e2;

            self.translate_expr(e1);
            self.translate_expr(e2);
            let r3 = self.builder.register_pop();
            let r2 = self.builder.register_pop();
            let r1 = self.builder.register_push_temp();
            match op {
                Op::Ne => self.builder.push_op(Instruction::Neq(r1, r2, r3)),
                Op::Add => self.builder.push_op(Instruction::Add(r1, r2, r3)),
                Op::Sub => self.builder.push_op(Instruction::Sub(r1, r2, r3)),
                Op::Mul => self.builder.push_op(Instruction::Mul(r1, r2, r3)),
                Op::Div => self.builder.push_op(Instruction::Div(r1, r2, r3)),
                Op::Eq => self.builder.push_op(Instruction::Eq(r1, r2, r3)),
                Op::Gt => self.builder.push_op(Instruction::Gt(r1, r2, r3)),
                Op::Lt => self.builder.push_op(Instruction::Lt(r1, r2, r3)),
                Op::Le => self.builder.push_op(Instruction::Le(r1, r2, r3)),
                Op::Ge => self.builder.push_op(Instruction::Ge(r1, r2, r3)),
                Op::And => self.builder.push_op(Instruction::And(r1, r2, r3)),
                Op::Or => self.builder.push_op(Instruction::Or(r1, r2, r3)),
                Op::BitOr => self.builder.push_op(Instruction::BitOr(r1, r2, r3)),
                Op::BitAnd => self.builder.push_op(Instruction::BitAnd(r1, r2, r3)),
                Op::BitXor => self.builder.push_op(Instruction::BitXor(r1, r2, r3)),
                Op::Shl => self.builder.push_op(Instruction::Shl(r1, r2, r3)),
                Op::Shr => self.builder.push_op(Instruction::Shr(r1, r2, r3)),
                Op::Isa => self.builder.push_op(Instruction::Isa(r1, r2, r3)),
                _ => unimplemented!(),
            }
            self.builder.register_clear(r2);
            self.builder.register_clear(r3);
        }
    }
}
