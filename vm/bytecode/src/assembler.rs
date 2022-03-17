use osmon_vm::opcodes::Instruction;
use crate::opcode::{Opcode,Size};
use crate::encode;

#[derive(Clone,Debug)]
pub struct Assembler {
    pub instructions: Vec<Instruction>,
    pub code: Vec<u8>,
}

impl Assembler {
    pub fn new(code: Vec<Instruction>) -> Assembler {
        Assembler {
            instructions: code,
            code: vec![]
        }
    }

    pub fn translate(&mut self) {
        let mut ip = 0;
        while ip < self.instructions.len() {
            let instruction = self.instructions[ip].clone();

            match instruction {
                Instruction::LoadInt(reg,val) => {
                    self.code.push(Opcode::LoadI);
                    self.code.push(reg as u8);
                    self.code.extend_from_slice(&encode!(val;i32));
                }
                Instruction::Move(reg,reg2) => {
                    self.code.push(Opcode::Move);
                    self.code.push(reg as u8);
                    self.code.push(reg2 as u8);
                }
                Instruction::LoadLong(reg,val) => {
                    self.code.push(Opcode::LoadL);
                    self.code.push(reg as u8);
                    self.code.extend_from_slice(&encode!(val;i64));
                }
                _ => unimplemented!(),
            }

            ip += 1;
        }
        self.code.push(self.instructions.len() as u8);
    }
}

