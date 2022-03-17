use osmon_vm::opcodes::Instruction;
use super::opcode::{Opcode,Size};
use super::decode;

#[derive(Clone,Debug)]
pub struct Parser<'a> {
    pub code: &'a [u8],
    pub parsed_code: Vec<Instruction>,
    pub ip: usize,
}


impl<'a> Parser<'a> {
    pub fn new(code: &'a [u8]) -> Parser<'a> {
        Parser {
            code,
            parsed_code: vec![],
            ip: 0,
        }
    }

    pub fn read_next(&mut self) -> u8 {
        if self.ip < self.code.len() {
            let op = self.code[self.ip].clone();

            self.ip += 1;
            op
        } else {

            return 0x0;
        }
    }
    pub fn parse(&mut self) -> Vec<Instruction> {
        let size = *self.code.last().unwrap() as usize;
        let mut ip = 0;
        while ip < size {
            if self.ip >= self.code.len() {
                break;
            }
            self.parse_opcode();
            ip += 1;

        }

        return self.parsed_code.clone();
    }

    pub fn parse_opcode(&mut self) {
        let op = &self.read_next();
        println!("{:?}",Opcode::to_string(op.clone()));
        match op {
            &Opcode::Move => {
                let r1 = self.read_next();
                let r2 = self.read_next();
                self.parsed_code.push(Instruction::Move(r1 as usize,r2 as usize));
            }

            &Opcode::LoadI => {

                let register = self.read_next() as usize;

                let array = {
                    let mut array = [0u8;Size::Int as usize];
                    let mut i = 0;
                    while i < Size::Int {
                        let idx = self.read_next();
                        array[i as usize] = idx;
                        i += 1;
                    }
                    array
                };

                let int = decode!(array;i32);
                self.parsed_code.push(Instruction::LoadInt(register,int));
            },

            &Opcode::LoadL => {
                let register = self.read_next() as usize;

                let array = {
                    let mut array = [0u8;Size::Long as usize];
                    for i in 0..Size::Long as usize {
                        array[i] = self.read_next();
                    }
                    array
                };

                let long = decode!(array;i64);
                self.parsed_code.push(Instruction::LoadLong(register,long));
            }
            &Opcode::LoadF => {
                let register = self.read_next() as usize;

                let array = {
                    let mut array = [0u8;Size::Float as usize];
                    for i in 0..Size::Float as usize {
                        array[i] = self.read_next();
                    }
                    array
                };

                let float = decode!(array;f32);
                self.parsed_code.push(Instruction::LoadFloat(register,float));
            }
            &Opcode::Label => {
                let label_id = self.read_next() as usize;

                self.parsed_code.push(Instruction::Label(label_id));
            }

            &Opcode::GotoF => {
                let reg = self.read_next();
                let lbl_id = self.read_next();

                self.parsed_code.push(Instruction::GotoF(reg as usize,lbl_id as usize));
            }

            &Opcode::Goto => {
                let lbl_id = self.read_next();

                self.parsed_code.push(Instruction::Goto(lbl_id as usize));
            }

            _ => {}
        }
    }
}
