use crate::{machine::Machine, object_pool::ObjectPool, value::Value};
use std::any::Any;
pub trait Object: Send + ObjectAddon {
    /// Initialized
    fn initialize(&mut self, _: &mut ObjectPool) {}
    fn call(&self, _m: &mut Machine, _args: Vec<Value>) -> Value {
        Value::Null
    }

    fn store_at(&self, _m: &mut Machine, _args: Vec<Value>, _rindex: usize) {
        println!("{:?}", _args);
        panic!("Cannot store_at, {}", self.to_String(_m));
    }

    fn load_at(&self, _m: &mut Machine, _args: Vec<Value>, _rindex: usize) {
        panic!("Cannot load_at. {:?}", self.to_String(_m));
    }

    fn as_any(&self) -> &dyn Any;

    fn as_any_mut(&mut self) -> &mut dyn Any;

    fn get_children(&self) -> Vec<usize>;
}

pub trait ObjectAddon {
    fn typename(&self, _m: &mut Machine) -> String {
        String::from("Object")
    }

    fn o_clone(&self, _m: &mut Machine) -> Value {
        panic!("Cannot clone!");
    }
    fn to_String(&self, _: &mut Machine) -> String {
        String::new()
    }

    fn as_bytes(&self, _: &mut Machine) -> Vec<u8> {
        Vec::new()
    }

    fn to_int(&self, _: &mut Machine) -> i32 {
        0
    }

    fn to_long(&self, _: &mut Machine) -> i64 {
        0
    }

    fn to_float(&self, _: &mut Machine) -> f32 {
        0.0
    }
    fn to_double(&self, _: &mut Machine) -> f64 {
        0.0
    }

    fn as_function(&self) -> &crate::function::Function {
        panic!()
    }

    fn eq(&self, _m: &mut Machine) -> bool {
        false
    }

    fn add(&self, _rhs: Value, _m: &mut Machine) -> Value {
        Value::Null
    }

    fn sub(&self, _rhs: Value, _m: &mut Machine) -> Value {
        Value::Null
    }

    fn div(&self, _rhs: Value, _m: &mut Machine) -> Value {
        Value::Null
    }

    fn not(&self, _m: &mut Machine) -> bool {
        false
    }

    fn isa(&self, s: String, m: &mut Machine) -> bool {
        self.typename(m) == s
    }
}
