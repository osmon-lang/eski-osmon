use std::collections::HashMap;
use osmon_vm::function::Function;
use osmon_vm::opcodes::Instruction;
use osmon_vm::value::Value;

#[derive(Debug,Clone)]
pub struct Context {
    pub builder: (),
}
