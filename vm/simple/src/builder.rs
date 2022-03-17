use osmon_vm::function::Function;
use std::collections::HashMap;
use osmon_vm::opcodes::Instruction;


///Simple function builder
#[derive(Debug)]
pub struct FunctionBuilder {
    pub argc: usize,
    pub regs_used: Vec<bool>,
    pub registers: Vec<usize>,
    pub nlocals: usize,
    pub ntemps: usize,
    pub locals: Vec<bool>,
    list: Vec<Instruction>,
}

impl FunctionBuilder {
    pub fn new(argc: usize) -> FunctionBuilder {
        let mut vec = vec![];
        for _ in 0..256 {
            vec.push(false);
        }




        FunctionBuilder {
            argc,
            list: vec![],
            regs_used: vec,
            registers: Vec::with_capacity(256),
            nlocals: argc,
            ntemps: 0,
            locals: vec![],
        }
    }

    pub fn create_function(&mut self) -> Function {
        self.list.push(Instruction::Ret0);

        let func = Function::from_instructions(self.list.clone(),self.argc);

        func
    }

    pub fn insert_op(&mut self,op: Instruction) {
        self.list.push(op);
    }

    pub fn iconst(&mut self, i: i32) -> usize {
        let register = self.register_push_temp();
        self.list.push(Instruction::LoadInt(register,i));
        register
    }

    pub fn lconst(&mut self, l: i64) -> usize {
        let register = self.register_push_temp();
        self.list.push(Instruction::LoadLong(register,l));
        register
    }

    pub fn dconst(&mut self, f: f64) -> usize {
        let register = self.register_push_temp();
        self.list.push(Instruction::LoadDouble(register,f));
        register
    }

    pub fn fconst(&mut self, f: f32) -> usize {
        let register = self.register_push_temp();
        self.list.push(Instruction::LoadFloat(register,f));
        register
    }


    pub fn register_pop_protect(&mut self,protect: bool) -> usize {
        let value = self.registers.pop().unwrap();
        if protect {
            self.regs_used[value] = true;
        } else if value >= self.nlocals {
            self.regs_used[value] = false;
        }

        if protect && value >= self.nlocals {
            self.locals[value] = true;
        }

        return value;
    }

    pub fn register_pop(&mut self) -> usize {
        self.register_pop_protect(false)
    }
    pub fn new_register(&mut self) -> usize  {
        for i in 0..255 {
            if self.regs_used[i] == false {
                self.regs_used[i] = true;
                return i;
            }
        }

        panic!("No registers availbale");
    }
    /// create temp register
    pub fn register_push_temp(&mut self) -> usize{
        let value = self.new_register();
        self.registers.push(value);
        self.nlocals += 1;
        if value > 256 {
            panic!("ERROR!");
        }

        return value;
    }

    pub fn register_is_temp(&self,nreg: usize) -> bool {
        return nreg >= self.nlocals;
    }

    pub fn register_push(&mut self,nreg: usize) -> usize {
        self.registers.push(nreg);

        if self.register_is_temp(nreg) {
            self.ntemps += 1;
        }
        nreg
    }


    pub fn first_temp_available(&mut self) -> usize {
        for i in 0..256 {
            if self.regs_used[i] == false {
                return i;
            }
        }

        println!("No available registers");
        return 0;
    }

    pub fn register_last(&self) -> usize {
        self.registers.last().unwrap().clone()
    }
}


pub trait Load<T> {
    fn load(f: &mut FunctionBuilder,v: T) -> usize;
}

impl Load<i32> for i32 {
    fn load(f: &mut FunctionBuilder,v: i32) -> usize {
        let reg = f.iconst(v);
        reg
    }
}


impl Load<i64> for i32 {
    fn load(f: &mut FunctionBuilder,v: i64) -> usize {
        let reg = f.lconst(v);
        reg
    }
}

impl Load<f32> for f32 {
    fn load(f: &mut FunctionBuilder,v: f32) -> usize {
        let reg = f.fconst(v);
        reg
    }
}

impl Load<f64> for f64 {
    fn load(f: &mut FunctionBuilder, v: f64) -> usize {
        let reg = f.dconst(v);
        reg
    }
}


pub trait InstBuilder<T: Load<T>> {

}
