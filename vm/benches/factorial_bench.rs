#![feature(test)]
extern crate test;

extern crate osmon_vm;

use self::opcodes::Instruction;
use osmon_vm::*;

use self::{function::Function, value::Value};

use self::test::Bencher;
#[bench]
fn factorial_bench(b: &mut Bencher)
{
    b.iter(|| {
        let mut machine = machine::Machine::new();

        use self::Instruction::*;

        let factorial_code = vec![
            LoadInt(2, 0),
            Eq(2, 1, 2),
            GotoF(2, 1),
            LoadInt(2, 1),
            Ret(2),
            Label(1),
            LoadGlobal(2, 3),
            LoadInt(5, 1),
            Sub(5, 1, 5),
            PushArg(5),
            PushArg(3),
            Call(3, 3, 1),
            Mul(3, 3, 1),
            Ret(3),
        ];

        let fun = Function::from(factorial_code);
        let fun_v = Value::Object(machine.pool.allocate(Box::new(fun)));
        machine.globals.insert(2, fun_v);

        let main_code = vec![
            LoadLong(1, 12),
            PushArg(1),
            LoadGlobal(2, 2),
            PushArg(2),
            Call(2, 2, 1),
            Ret(2),
        ];

        let fun = Function::from(main_code);
        let fun_v = Value::Object(machine.pool.allocate(Box::new(fun)));
        let v = machine.invoke(fun_v, vec![]);
        let int = if let Value::Long(i) = v {
            i
        } else {
            panic!("");
        };
        assert_eq!(479001600, int);
    })
}
