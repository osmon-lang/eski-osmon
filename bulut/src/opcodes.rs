use colored;

use self::colored::Colorize;

#[derive(Clone)]
pub enum Instruction {
    LoadString(usize, String),

    /// LoadBool R(A) = B
    ///
    /// Loading bool value B to register A
    LoadBool(usize, bool),

    ///LoadInt R(A) = B
    ///
    /// Loading integer value B to register A
    LoadInt(usize, i32),
    ///LoadLong R(A) = B
    ///
    /// Loading long value B to register A
    LoadLong(usize, i64),
    ///LoadFloat R(A) = B
    ///
    /// Loading float value B to register A
    LoadFloat(usize, f32),
    /// LoadDouble R(A) = B
    ///
    /// Loading double value B to register A
    LoadDouble(usize, f64),

    /// LoadConst R(A) = C(B)
    ///
    /// Load constant from object pool to register A
    LoadConst(usize, usize),
    /// LoadGlobal R(A) = G(B)
    ///
    /// Load global value B into register A
    LoadGlobal(usize, usize),
    /// LoadAt R(A) = R(B)\[C\]
    ///
    /// Load C from B and store in A
    LoadAt(usize, usize, usize),
    /// LoadSuper R(A) = R(B)\[C\]
    ///
    /// Load C from B and store in A
    LoadSuper(usize, usize, usize),
    /// Move R(A) = R(B)
    ///
    /// Move register
    Move(usize, usize),
    /// Store R(B)\[C\] = A
    ///
    /// Store A into R(B)\[C\]
    Store(usize, usize, usize),
    StoreAt(usize, usize, usize),
    /// StoreGlobal G(A) = R(B)
    ///
    /// Store global
    StoreGlobal(usize, usize),
    /// Jump IP
    Jump(usize),
    /// Jump (R(A) == false ? ip = B : continue)
    JumpF(usize, usize),

    /// Goto
    ///
    /// Same as Jump instructions, but uses labels
    Goto(usize),
    GotoF(usize, usize),

    /// Push value from R(A) to arguments stack
    LoadArg(usize),
    /// R(A) = B(Args), C - Arg count, args poped from arg stack
    Call(usize, usize, usize),

    Isa(usize, usize, usize),

    Not(usize, usize),
    ///Add R(A) = R(B) + R(C)
    Add(usize, usize, usize),
    ///Sub R(A) = R(B) - R(C)
    Sub(usize, usize, usize),
    ///Mul R(A) = R(B) * R(C)
    Mul(usize, usize, usize),
    ///Div R(A) = R(B) / R(C)
    Div(usize, usize, usize),
    Rem(usize, usize, usize),
    ///Gt R(A) = R(B) > R(C)
    Gt(usize, usize, usize),
    ///Lt R(A) = R(B) < R(C)
    Lt(usize, usize, usize),
    /// Ge R(A) = R(B) >= R(C)
    Ge(usize, usize, usize),
    /// Le R(A) = R(B) <= R(C)
    Le(usize, usize, usize),

    /// Eq R(A) = R(B) == R(C)
    Eq(usize, usize, usize),
    Neq(usize, usize, usize),
    /// Ret0
    ///
    /// return null value
    Ret0,
    /// Ret R(A)
    ///
    /// return value from R(A)
    Ret(usize),

    /// Create label with id A
    Label(usize),

    Shr(usize, usize, usize),
    Shl(usize, usize, usize),
    BitOr(usize, usize, usize),
    BitXor(usize, usize, usize),
    BitAnd(usize, usize, usize),
    And(usize, usize, usize),
    Or(usize, usize, usize),
}

use std::fmt;
impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use self::Instruction::*;

        match self {
            Neq(dest, r1, r2) => write!(f, "Neq {} {} {}", dest, r1, r2),
            Isa(dest, r1, r2) => write!(f, "Isa {} {} {}", dest, r1, r2),
            Not(r1, r2) => write!(f, "Not {} {}", r1, r2),
            Add(r3, r1, r2) => write!(f, "Add {} {} {}", r3, r1, r2),
            Sub(r3, r1, r2) => write!(f, "Sub {} {} {}", r3, r1, r2),
            Div(r3, r1, r2) => write!(f, "Div {} {} {}", r3, r1, r2),
            Mul(r3, r1, r2) => write!(f, "Mul {} {} {}", r3, r1, r2),
            Rem(r3, r1, r2) => write!(f, "Rem {} {} {}", r3, r1, r2),
            Gt(r3, r1, r2) => write!(f, "Gt {} {} {} ", r3, r1, r2),
            Lt(r3, r1, r2) => write!(f, "Lt {} {} {}", r3, r1, r2),
            Le(r3, r1, r2) => write!(f, "Le {} {} {}", r3, r1, r2),
            Ge(r3, r1, r2) => write!(f, "Ge {} {} {}", r3, r1, r2),
            Eq(r3, r1, r2) => write!(f, "Eq {} {} {}", r3, r1, r2),
            Ret0 => write!(f, "Ret0"),
            Ret(r1) => write!(f, "Ret {}", r1),
            Goto(label_id) => write!(f, "Goto {}", label_id),
            GotoF(r1, label_id) => write!(f, "GotoF {} {}", r1, label_id),
            Jump(ip) => write!(f, "Jump {}", ip),
            JumpF(r1, ip) => write!(f, "JumpF {} {}", r1, ip),
            LoadConst(r1, object_id) => write!(f, "LoadConst {} {}", r1, object_id),
            LoadGlobal(r1, global) => write!(f, "LoadGlobal {} {}", r1, global),
            LoadInt(r1, int) => write!(f, "LoadInt {} {}", r1, int),
            LoadLong(r1, long) => write!(f, "LoadLong {} {}", r1, long),
            LoadFloat(r1, float) => write!(f, "LoadFloat {} {}", r1, float),
            LoadDouble(r1, double) => write!(f, "LoadDouble {} {}", r1, double),
            LoadBool(r1, bool) => write!(f, "LoadBool {} {}", r1, bool),
            LoadString(r1, str) => write!(f, "LoadString {} \"{}\"", r1, str),
            StoreGlobal(r1, global) => write!(f, "StoreGlobal {} {}", r1, global),
            StoreAt(r1, r2, r3) => write!(f, "StoreAt {} {} {}", r1, r2, r3),
            Store(r1, r2, r3) => write!(f, "Store {} {} {}", r1, r2, r3),
            LoadAt(r1, r2, r3) => write!(f, "LoadAt {} {} {}", r1, r2, r3),
            BitAnd(r3, r1, r2) => write!(f, "BitAnd {} {} {}", r3, r1, r2),
            BitOr(r3, r1, r2) => write!(f, "BitOr {} {} {}", r3, r1, r2),
            BitXor(r3, r1, r2) => write!(f, "BitXor {} {} {}", r3, r1, r2),
            Or(r3, r1, r2) => write!(f, "Or {} {} {}", r3, r1, r2),
            And(r3, r1, r2) => write!(f, "And {} {} {}", r3, r1, r2),
            Shr(r3, r1, r2) => write!(f, "Shr {} {} {}", r3, r1, r2),
            Shl(r3, r1, r2) => write!(f, "Shl {} {} {}", r3, r1, r2),
            Label(id) => write!(f, "Label {}", id),
            Call(r3, r2, r1) => write!(f, "Call {} {} {}", r3, r2, r1),
            LoadArg(r1) => write!(f, "LoadArg {}", r1),
            Move(r1, r2) => write!(f, "Move {} {}", r1, r2),
            LoadSuper(r3, r2, r1) => write!(f, "LoadSuper {} {} {}", r3, r2, r1),
        }
    }
}

impl fmt::Debug for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use self::Instruction::*;

        match self {
            Neq(dest, r1, r2) => write!(f, "Neq {} {} {}", dest, r1, r2),
            Isa(dest, r1, r2) => write!(f, "Isa {} {} {}", dest, r1, r2),
            Not(r1, r2) => write!(f, "Not {} {}", r1, r2),
            Add(r3, r1, r2) => write!(f, "Add {} {} {}", r3, r1, r2),
            Sub(r3, r1, r2) => write!(f, "Sub {} {} {}", r3, r1, r2),
            Div(r3, r1, r2) => write!(f, "Div {} {} {}", r3, r1, r2),
            Mul(r3, r1, r2) => write!(f, "Mul {} {} {}", r3, r1, r2),
            Rem(r3, r1, r2) => write!(f, "Rem {} {} {}", r3, r1, r2),
            Gt(r3, r1, r2) => write!(f, "Gt {} {} {} ", r3, r1, r2),
            Lt(r3, r1, r2) => write!(f, "Lt {} {} {}", r3, r1, r2),
            Le(r3, r1, r2) => write!(f, "Le {} {} {}", r3, r1, r2),
            Ge(r3, r1, r2) => write!(f, "Ge {} {} {}", r3, r1, r2),
            Eq(r3, r1, r2) => write!(f, "Eq {} {} {}", r3, r1, r2),
            Ret0 => write!(f, "Ret0"),
            Ret(r1) => write!(f, "Ret {}", r1),
            Goto(label_id) => write!(f, "Goto {}", label_id),
            GotoF(r1, label_id) => write!(f, "GotoF {} {}", r1, label_id),
            Jump(ip) => write!(f, "Jump {}", ip),
            JumpF(r1, ip) => write!(f, "JumpF {} {}", r1, ip),
            LoadConst(r1, object_id) => write!(f, "LoadConst {} {}", r1, object_id),
            LoadGlobal(r1, global) => write!(f, "LoadGlobal {} {}", r1, global),
            LoadInt(r1, int) => write!(f, "LoadInt {} {}", r1, int),
            LoadLong(r1, long) => write!(f, "LoadLong {} {}", r1, long),
            LoadFloat(r1, float) => write!(f, "LoadFloat {} {}", r1, float),
            LoadDouble(r1, double) => write!(f, "LoadDouble {} {}", r1, double),
            LoadBool(r1, bool) => write!(f, "LoadBool {} {}", r1, bool),
            LoadString(r1, str) => write!(f, "LoadString {} \"{}\"", r1, str),
            StoreGlobal(r1, global) => write!(f, "StoreGlobal {} {}", r1, global),
            StoreAt(r1, r2, r3) => write!(f, "StoreAt {} {} {}", r1, r2, r3),
            Store(r1, r2, r3) => write!(f, "Store {} {} {}", r1, r2, r3),
            LoadAt(r1, r2, r3) => write!(f, "LoadAt {} {} {}", r1, r2, r3),
            BitAnd(r3, r1, r2) => write!(f, "BitAnd {} {} {}", r3, r1, r2),
            BitOr(r3, r1, r2) => write!(f, "BitOr {} {} {}", r3, r1, r2),
            BitXor(r3, r1, r2) => write!(f, "BitXor {} {} {}", r3, r1, r2),
            Or(r3, r1, r2) => write!(f, "Or {} {} {}", r3, r1, r2),
            And(r3, r1, r2) => write!(f, "And {} {} {}", r3, r1, r2),
            Shr(r3, r1, r2) => write!(f, "Shr {} {} {}", r3, r1, r2),
            Shl(r3, r1, r2) => write!(f, "Shl {} {} {}", r3, r1, r2),
            Label(id) => write!(f, "Label {}", id),
            Call(r3, r2, r1) => write!(f, "Call {} {} {}", r3, r2, r1),
            LoadArg(r1) => write!(f, "LoadArg {}", r1),
            Move(r1, r2) => write!(f, "Move {} {}", r1, r2),
            LoadSuper(r3, r2, r1) => write!(f, "LoadSuper {} {} {}", r3, r2, r1),
        }
    }
}

///Trait used for print Vec\<Instruction\>

pub trait DebugCode {
    #[allow(non_snake_case)]
    fn toString(&self) -> String;
}

impl DebugCode for Vec<Instruction> {
    fn toString(&self) -> String {
        let mut str = String::new();
        for i in 0..self.len() {
            str.push_str(&format!("{:04} {}", i, format!("{}", self[i]).white()));
            str.push('\n');
        }
        str
    }
}
/// Stores instructions
#[derive(Clone, Debug)]
pub struct CodeBlock {
    pub code: Vec<Instruction>,
    pub ip: usize,
}

impl CodeBlock {
    /// Create new instance of CodeBlock
    pub fn new(ins: Vec<Instruction>) -> CodeBlock {
        CodeBlock { code: ins, ip: 0 }
    }
}
