use self::float_duration::FloatDuration;
use crate::{builtins::*, class::Class};
use bulut::{function::Function, machine::Machine, value::Value};
use float_duration;
use std::{cell::UnsafeCell, collections::HashMap, time::Instant};

pub fn time(m: &mut Machine, _: Vec<Value>) -> Value {
    let now = Instant::now();
    let duration = FloatDuration::from_std(now.elapsed());

    let str = format!("{}", duration);
    let obj = Value::Object(m.pool.allocate(Box::new(str)));
    return obj;
}

pub fn system_class(m: &mut Machine) -> Class {
    let mut fields = HashMap::new();
    let f = Function::from_native(Box::new(print));
    fields.insert(
        "yoz".to_owned(),
        Value::Object(m.pool.allocate(Box::new(f))),
    );
    fields.insert(
        "oqi".to_owned(),
        Value::Object(
            m.pool
                .allocate(Box::new(Function::from_native(Box::new(readln)))),
        ),
    );
    fields.insert(
        "vaqt".to_owned(),
        Value::Object(
            m.pool
                .allocate(Box::new(Function::from_native(Box::new(time)))),
        ),
    );
    Class {
        name: String::from("Tizim"),
        fields: UnsafeCell::new(fields),
    }
}

pub fn unary_minus(m: &mut Machine) -> Value {
    use bulut::opcodes::Instruction::*;

    let code = vec![LoadLong(2, 0), Move(3, 1), Sub(3, 2, 3), Ret(3)];

    let func = Function::from_instructions(code, 1);
    Value::Object(m.pool.allocate(Box::new(func)))
}

pub fn int_class() -> Class {
    Class {
        name: String::from("Int"),
        fields: UnsafeCell::new(HashMap::new()),
    }
}

pub fn float_class() -> Class {
    Class {
        name: String::from("Float"),
        fields: UnsafeCell::new(HashMap::new()),
    }
}

pub fn str_class() -> Class {
    Class {
        name: String::from("Str"),
        fields: UnsafeCell::new(HashMap::new()),
    }
}
