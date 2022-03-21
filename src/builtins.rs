use bulut::{
    function::Function,
    machine::Machine,
    object::{Object, ObjectAddon},
    object_pool::ObjectPool,
    value::Value,
};

use std::{cell::RefCell, io::stdin};

#[derive(Clone)]
pub struct Array {
    elements: RefCell<Vec<Value>>,
}

pub fn array_pop(m: &mut Machine, args: Vec<Value>) -> Value {
    if let Value::Object(id) = &args[1] {
        let obj = m.pool.get(*id).as_any();

        let array = if let Some(array) = obj.downcast_ref::<Array>() {
            array
        } else {
            panic!("Not a Array object!");
        };

        let v = array.pop();
        return v;
    } else {
        panic!("Array::pop expects Array as argument!");
    }
}

pub fn array_push(m: &mut Machine, args: Vec<Value>) -> Value {
    if let Value::Object(id) = &args[0] {
        let obj = m.pool.get(*id).as_any();

        let array = if let Some(array) = obj.downcast_ref::<Array>() {
            array
        } else {
            panic!("Not a Array object!");
        };

        let v = args[1];
        array.push(v);
        return Value::Null;
    } else {
        panic!("Array::push expects Array as argument!");
    }
}

pub fn array_size(m: &mut Machine, args: Vec<Value>) -> Value {
    if let Value::Object(id) = &args[0] {
        let obj = m.pool.get(*id).as_any();

        let array = if let Some(array) = obj.downcast_ref::<Array>() {
            array
        } else {
            panic!("Not a Array object!");
        };
        return Value::Int(array.elements.borrow().len() as i32);
    } else {
        panic!("Array::size expects Array as argument!");
    }
}

pub fn array_get(m: &mut Machine, args: Vec<Value>) -> Value {
    if let Value::Object(id) = &args[0] {
        let obj = m.pool.get(*id).as_any();
        let idx = match &args[1] {
            Value::Int(integer) => *integer as usize,
            Value::Long(long) => *long as usize,
            _ => panic!("Array::get expects Long or Int value as index"),
        };
        let array = if let Some(array) = obj.downcast_ref::<Array>() {
            array
        } else {
            panic!("Not a Array object!");
        };
        return array.get(idx);
    } else {
        panic!("Array::get expects Array as argument!");
    }
}

pub fn array_set(m: &mut Machine, args: Vec<Value>) -> Value {
    if let Value::Object(id) = &args[0] {
        let obj = m.pool.get(*id).as_any();
        let idx = match &args[1] {
            Value::Int(integer) => *integer as usize,
            Value::Long(long) => *long as usize,
            _ => panic!("Array::set expects Long or Int value as index"),
        };
        let array = if let Some(array) = obj.downcast_ref::<Array>() {
            array
        } else {
            panic!("Not a Array object!");
        };

        return array.get(idx);
    } else {
        panic!("Array::set expects Array as argument!");
    }
}

impl Array {
    pub fn new() -> Array {
        Array {
            elements: RefCell::new(Vec::new()),
        }
    }

    pub fn push(&self, v: Value) {
        self.elements.borrow_mut().push(v);
    }

    pub fn pop(&self) -> Value {
        let mut elements = self.elements.borrow_mut();
        let value = {
            let value = elements.pop();
            if value.is_some() {
                value.unwrap()
            } else {
                Value::Null
            }
        };
        value
    }

    pub fn set(&self, idx: usize, v: Value) {
        self.elements.borrow_mut()[idx] = v;
    }

    pub fn get(&self, idx: usize) -> Value {
        self.elements.borrow()[idx]
    }
}

use std::any::Any;

impl ObjectAddon for Array {
    fn to_String(&self, m: &mut Machine) -> String {
        let elements = self.elements.borrow();

        let mut string = String::new();
        string.push_str("[");
        let mut i = 0;
        while i < elements.len() {
            string.push_str(&elements[i].to_String(m));
            if i != elements.len() - 1 {
                string.push_str(",");
            }
            i += 1;
        }
        string.push_str("]");

        string
    }
}

impl Object for Array {
    fn initialize(&mut self, _: &mut ObjectPool) {}
    fn store_at(&self, m: &mut Machine, args: Vec<Value>, _rindex: usize) {
        let idx = args[1];
        let value = args[2];

        let idx = idx.to_int(m) as usize;
        self.elements.borrow_mut()[idx] = value;
    }

    fn load_at(&self, m: &mut Machine, args: Vec<Value>, rindex: usize) {
        let _this = args[0];
        if let Value::Object(id) = &args[1] {
            let str: &str = &m.pool.get(*id).to_String(m);

            match str {
                "pop" => {
                    let function = Function::from_native(Box::new(array_pop));
                    let function_id = Value::Object(m.pool.allocate(Box::new(function)));
                    m.set(rindex, function_id);
                }
                "push" => {
                    let function = Function::from_native(Box::new(array_push));
                    let function_id = Value::Object(m.pool.allocate(Box::new(function)));
                    m.set(rindex, function_id);
                }
                "set" => {
                    let function = Function::from_native(Box::new(array_pop));
                    let function_id = Value::Object(m.pool.allocate(Box::new(function)));
                    m.set(rindex, function_id);
                }
                "get" => {
                    let function = Function::from_native(Box::new(array_get));
                    let function_id = Value::Object(m.pool.allocate(Box::new(function)));
                    m.set(rindex, function_id);
                }
                "size" => {
                    let function = Function::from_native(Box::new(array_size));
                    let function_id = Value::Object(m.pool.allocate(Box::new(function)));
                    m.set(rindex, function_id);
                }
                v => panic!("{:?}", v),
            }
            return;
        }
        let elements = self.elements.borrow();
        if let Value::Int(int) = &args[1] {
            let v = elements.get(*int as usize).unwrap();

            m.set(rindex, v.clone());
        }
        if let Value::Long(long) = &args[1] {
            let v = elements.get(*long as usize).unwrap();
            m.set(rindex, v.clone());
        }
    }

    fn as_any(&self) -> &dyn Any {
        self as &dyn Any
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self as &mut dyn Any
    }

    fn get_children(&self) -> Vec<usize> {
        Vec::new()
    }
}

pub fn new_array(m: &mut Machine, args: Vec<Value>) -> Value {
    let array = Array::new();
    for i in 1..args.len() {
        array.push(args[args.len() - i]);
    }
    let object = Value::Object(m.pool.allocate(Box::new(array)));
    object
}

pub fn concat(m: &mut Machine, args: Vec<Value>) -> Value {
    let mut buffer = String::new();
    for i in 1..args.len() {
        buffer.push_str(&args[i].to_String(m));
    }
    let object = Value::Object(m.pool.allocate(Box::new(buffer)));
    object
}

pub fn print(m: &mut Machine, args: Vec<Value>) -> Value {
    for i in 1..args.len() {
        let str = args[i].to_String(m);

        print!("{}", str);
    }
    print!("\n");
    Value::Null
}

pub fn readln(m: &mut Machine, _args: Vec<Value>) -> Value {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();
    Value::Object(m.pool.allocate(Box::new(buffer)))
}
