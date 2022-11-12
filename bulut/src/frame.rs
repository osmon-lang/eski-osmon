use crate::{opcodes::Instruction, value::Value};

///CallFrame
/// Stores register values, instructions, registers and arguments stack
#[derive(Clone, Debug, Default)]
pub struct CallFrame {
    /// pointer to current block
    pub ip: usize,
    /// Instructions
    pub code: Vec<Instruction>,
    /// registers stored in stack, default size of `stack` is 256
    pub stack: Vec<Value>,
    /// `arguments stack`: used by Call instruction
    pub arg_stack: Vec<Value>,
}

/// CallStack
/// Stores frames
#[derive(Clone, Debug)]
pub struct CallStack {
    pub frames: Vec<CallFrame>,
    pub n_frames: usize,
    pub limit: Option<usize>,
}

impl CallStack {
    pub fn new(len: usize) -> CallStack {
        let mut frames = Vec::with_capacity(len);
        for _ in 0..len {
            frames.push(CallFrame::new());
        }

        CallStack {
            frames,
            n_frames: 1,
            limit: None,
        }
    }

    pub fn push(&mut self) {
        if self.n_frames >= self.frames.len() {
            panic!("Virtual stack overflow");
        }
        if let Some(limit) = self.limit {
            if self.n_frames >= limit {
                panic!("Maximum stack depth exceeded");
            }
        }

        self.n_frames += 1;
    }

    /// Reset last frame
    pub fn pop(&mut self) {
        if self.n_frames == 0 {
            panic!("Virtual stack underflow");
        }
        self.frames[self.n_frames - 1].reset();
        self.n_frames -= 1;
    }
    /// Get top frame
    pub fn top(&self) -> &CallFrame {
        if self.n_frames == 0 {
            panic!("Virtual stack underflow");
        }
        &self.frames[self.n_frames - 1]
    }
    /// Get &mut CallFrame
    pub fn top_mut(&mut self) -> &mut CallFrame {
        if self.n_frames == 0 {
            panic!("Virtual stack underflow");
        }
        &mut self.frames[self.n_frames - 1]
    }
}

///Fiber
/// Should be used in future
///
/// Fiber must need to store upvalues and have a object pool
#[derive(Clone, Debug)]
pub struct Fiber {
    pub frames: Vec<CallFrame>,
}

impl CallFrame {
    pub fn new() -> CallFrame {
        let mut vec = Vec::with_capacity(256);
        for _ in 0..256 {
            vec.push(Value::Null);
        }

        CallFrame {
            ip: 0,

            code: vec![],
            stack: vec,
            arg_stack: vec![],
        }
    }

    pub fn get(&self, r: usize) -> Value {
        self.stack[r]
    }

    pub fn set(&mut self, r: usize, v: Value) {
        self.stack[r] = v;
    }

    pub fn init_with_args(&mut self, args: &[Value]) {
        for arg in args {
            self.arg_stack.push(*arg);
        }
    }

    pub fn reset(&mut self) {
        for i in 0..self.stack.len() {
            self.stack[i] = Value::Null;
        }
        self.arg_stack.clear();
        self.code.clear();
    }

    pub fn jit_run(&mut self) -> Value {
        Value::Null
    }
}
