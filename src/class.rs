use bulut::{
    machine::Machine,
    object::{Object, ObjectAddon},
    object_pool::ObjectPool,
    value::Value,
};

use std::{any::Any, cell::UnsafeCell, collections::HashMap};

#[derive(Debug)]
pub struct Class {
    pub name: String,
    pub fields: UnsafeCell<HashMap<String, Value>>,
}

impl Clone for Class {
    fn clone(&self) -> Class {
        let fields = unsafe { (*self.fields.get()).clone() };
        Class {
            name: self.name.clone(),
            fields: UnsafeCell::new(fields),
        }
    }
}

impl Class {
    pub fn new() -> Class {
        Class {
            name: String::from("<uninitialized>"),
            fields: UnsafeCell::new(HashMap::new()),
        }
    }
}

impl ObjectAddon for Class {
    fn o_clone(&self, m: &mut Machine) -> Value {
        let c = self.clone();

        let v = Value::Object(m.pool.allocate(Box::new(c)));
        v
    }
    fn typename(&self, _: &mut Machine) -> String {
        return self.name.clone();
    }

    fn to_String(&self, _m: &mut Machine) -> String {
        let fields: &HashMap<String, Value> = unsafe { &*self.fields.get() };
        let mut string = String::new();
        string.push_str(&format!("class {} {{ \n", self.name));
        for (k, v) in fields.iter() {
            string.push_str(&format!(
                "\tvar {} = {};\n",
                k.to_String(_m),
                v.to_String(_m)
            ));
        }
        string.push_str("}");

        string
    }
}
impl Object for Class {
    fn as_any(&self) -> &dyn Any {
        self as &dyn Any
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self as &mut dyn Any
    }
    fn initialize(&mut self, _p: &mut ObjectPool) {}

    fn get_children(&self) -> Vec<usize> {
        Vec::new()
    }

    fn call(&self, m: &mut Machine, args: Vec<Value>) -> Value {
        let class = if let Value::Object(id) = args[0] {
            let obj = m.pool.get(id);

            obj.as_any().downcast_ref::<Class>().unwrap()
        } else {
            panic!("Qiymat::Obyekt kutilgandi");
        };

        let ret = {
            let fields = unsafe { &mut *class.fields.get() };
            let field = fields.get("init").expect("Initisializatorlar topilmadi");
            let mut args = args.clone();
            args[0] = class.o_clone(m);
            let v = m.invoke(*field, args);
            m.stack.pop();
            v
        };
        ret
    }

    fn store_at(&self, m: &mut Machine, args: Vec<Value>, _: usize) {
        let fname = args[1].to_String(m);
        let fields = unsafe { &mut *self.fields.get() };
        fields.insert(fname, args[2]);
    }

    fn load_at(&self, m: &mut Machine, args: Vec<Value>, rindex: usize) {
        let _this = args[0];
        if let Value::Object(id) = args[1] {
            let str = m.pool.get(id).to_String(m);
            let fields = unsafe { &*self.fields.get() };
            let field = fields.get(&str).expect(&format!("Ushbu {} topilmadi", str));

            m.set(rindex, *field);
        }
        if let Value::Int(_) = args[1] {
            let fields = unsafe { &*self.fields.get() };
            let field = fields
                .get("__get__")
                .expect("Ushbu klassda __get__ metodi mavjud emas");
            let v = m.invoke(*field, args);
            m.set(rindex, v);
        }
    }
}