extern crate osmon_bytecode;
extern crate bulut;
use osmon_bytecode::parser::Parser;
use osmon_bytecode::assembler::Assembler;
use bulut::opcodes::Instruction;

fn main() {
    let mut assembler = Assembler::new(vec![
        Instruction::LoadInt(1,12),
        Instruction::Move(1,2),
    ]);

    assembler.translate();

    println!("{:?}",assembler.code);

    let mut parser = Parser::new(&assembler.code);
    let code = parser.parse();
    println!("{:?}",code);

}
