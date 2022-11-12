use crate::{machine::Machine, object::*, object_pool::ObjectPool};

impl ObjectAddon for String {
    fn typename(&self, _m: &mut Machine) -> String {
        String::from("Str")
    }

    fn to_String(&self, _: &mut Machine) -> String {
        self.clone()
    }

    fn to_int(&self, _: &mut Machine) -> i32 {
        self.parse::<i32>().unwrap()
    }
    fn to_long(&self, _: &mut Machine) -> i64 {
        self.parse::<i64>().unwrap()
    }
    fn to_float(&self, _: &mut Machine) -> f32 {
        self.parse::<f32>().unwrap()
    }
    fn to_double(&self, _: &mut Machine) -> f64 {
        self.parse::<f64>().unwrap()
    }
}

use std::any::Any;

impl Object for String {
    fn initialize(&mut self, _: &mut ObjectPool) {}
    fn as_any(&self) -> &dyn Any {
        self as &dyn Any
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self as &mut dyn Any
    }
    fn get_children(&self) -> Vec<usize> {
        vec![]
    }
}
