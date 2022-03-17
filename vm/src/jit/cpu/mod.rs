pub mod x64;

pub const REG_COUNT: usize = 16;

pub struct State {
    pub pc: usize,
    pub sp: usize,
    pub ra: usize,

    pub regs: [usize; REG_COUNT],
}

use std::fmt;

impl fmt::Debug for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        println!("State:");
        println!("\t pc = {:?}", self.pc as *const u8);
        println!("\t sp = {:?}", self.sp as *const u8);
        println!("\t ra = {:?}", self.ra as *const u8);
        for (ind, &val) in self.regs.iter().enumerate() {
            println!("R[{:2}] = {:-20?} {:-20}", ind, val as *const u8, val)
        }
        write!(f, "")
    }
}

/// Register
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Reg(pub u8);

impl From<Reg> for u32 {
    fn from(reg: Reg) -> u32 {
        reg.0 as u32
    }
}

/// Float register
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct FReg(pub u8);

impl From<FReg> for u32 {
    fn from(reg: FReg) -> u32 {
        reg.0 as u32
    }
}

pub enum Memory {
    Local(i32),
    Base(Reg, i32),
    Index(Reg, Reg, i32, i32),
    Offset(Reg, i32, i32),
}
