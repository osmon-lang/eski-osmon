#[derive(Clone, Debug, Copy)]
pub enum Value {
    /// Integer or i32 in Rust
    Int(i32),
    /// Long or i64 in Rust
    Long(i64),
    /// Float or f32 in Rust
    Float(f32),
    /// Double or f64 in Rust
    Double(f64),
    /// Pointer to object in pool
    Object(usize),
    /// Null reference
    Null,
    /// Boolean
    Bool(bool),
}

use crate::{machine::Machine, object::ObjectAddon};

impl ObjectAddon for Value {
    fn typename(&self, m: &mut Machine) -> String {
        match self {
            Value::Bool(_) => String::from("Bool"),
            Value::Double(_) => String::from("Float"),
            Value::Long(_) => String::from("Int"),
            Value::Object(id) => {
                let obj = m.pool.get(*id);
                obj.typename(m)
            }
            Value::Null => String::from("null"),
            _ => unimplemented!(),
        }
    }

    fn o_clone(&self, m: &mut Machine) -> Value {
        match self {
            Value::Object(id) => {
                let obj = m.pool.get(*id);
                obj.o_clone(m)
            }
            v => *v,
        }
    }
    fn to_String(&self, m: &mut Machine) -> String {
        match self {
            Value::Double(d) => d.to_string(),
            Value::Float(f) => f.to_string(),
            Value::Bool(b) => {
                if *b {
                    "true".to_string()
                } else {
                    "false".to_string()
                }
            }
            Value::Int(i) => i.to_string(),
            Value::Long(i) => i.to_string(),
            Value::Null => "null".to_string(),
            Value::Object(id) => {
                let obj = m.pool.get(*id);
                obj.to_String(m)
            }
        }
    }

    fn as_bytes(&self, m: &mut Machine) -> Vec<u8> {
        let string: String = match self {
            Value::Double(d) => d.to_string(),
            Value::Float(f) => f.to_string(),
            Value::Bool(b) => {
                if *b {
                    "true".to_string()
                } else {
                    "false".to_string()
                }
            }
            Value::Int(i) => i.to_string(),
            Value::Long(i) => i.to_string(),
            Value::Null => "null".to_string(),
            Value::Object(id) => {
                let obj = m.pool.get(*id);
                obj.to_String(m)
            }
        };
        string.into_bytes()
    }

    fn to_int(&self, m: &mut Machine) -> i32 {
        match self {
            Value::Double(d) => *d as i32,
            Value::Float(f) => *f as i32,
            Value::Bool(b) => {
                if *b {
                    1
                } else {
                    0
                }
            }
            Value::Int(i) => *i,
            Value::Long(i) => *i as i32,
            Value::Null => 0,
            Value::Object(id) => {
                let obj = m.pool.get(*id);
                obj.to_int(m)
            }
        }
    }

    fn to_long(&self, m: &mut Machine) -> i64 {
        match self {
            Value::Double(d) => *d as i64,
            Value::Float(f) => *f as i64,
            Value::Bool(b) => {
                if *b {
                    1
                } else {
                    0
                }
            }
            Value::Int(i) => i64::from(*i),
            Value::Long(i) => *i,
            Value::Null => 0,
            Value::Object(id) => {
                let obj = m.pool.get(*id);
                obj.to_long(m)
            }
        }
    }

    fn to_float(&self, m: &mut Machine) -> f32 {
        match self {
            Value::Double(d) => *d as f32,
            Value::Float(f) => *f,
            Value::Bool(b) => {
                if *b {
                    1.0
                } else {
                    0.0
                }
            }
            Value::Int(i) => *i as f32,
            Value::Long(i) => *i as f32,
            Value::Null => 0.0,
            Value::Object(id) => {
                let obj = m.pool.get(*id);
                obj.to_float(m)
            }
        }
    }

    fn to_double(&self, m: &mut Machine) -> f64 {
        match self {
            Value::Double(d) => *d,
            Value::Float(f) => f64::from(*f),
            Value::Bool(b) => {
                if *b {
                    1.0
                } else {
                    0.0
                }
            }
            Value::Int(i) => f64::from(*i),
            Value::Long(i) => *i as f64,
            Value::Null => 0.0,
            Value::Object(id) => {
                let obj = m.pool.get(*id);
                obj.to_double(m)
            }
        }
    }

    fn not(&self, _m: &mut Machine) -> bool {
        match self {
            Value::Null => true,
            Value::Int(i) => *i == 0,
            Value::Long(l) => *l == 0,
            Value::Double(b) => *b == 0.0,
            Value::Float(f) => *f == 0.0,
            Value::Bool(b) => !b,
            _ => unimplemented!(),
        }
    }
}
