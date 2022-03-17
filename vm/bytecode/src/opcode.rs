pub mod Opcode {
    pub const LoadI: u8 = 0x1;
    pub const LoadF: u8 = 0x2;
    pub const LoadL: u8 = 0x3;
    pub const LoadD: u8 = 0x4;

    pub const LoadG: u8 = 0xa1;
    pub const LoadAt: u8 = 0xa2;
    pub const StoreAt: u8 = 0xa3;
    pub const Ret: u8 = 0xa4;
    pub const Ret0: u8 = 0xa5;
    pub const Call: u8 = 0xa6;
    pub const StoreG: u8 = 0xa7;
    pub const Move: u8 = 0xa8;

    pub const Label: u8 = 0xa9;
    pub const Goto: u8 = 0xe1;
    pub const GotoT: u8 = 0xe2;
    pub const GotoF: u8 = 0xe3;

    pub fn to_string<'a>(op: u8) -> &'a str {
        match op {
            LoadI => "LoadI",
            Move => "Move",
            _ => "",
        }
    }
}

pub mod Size {
    pub const Float: u32 = ::std::mem::size_of::<f32>() as u32;
    pub const Double: u32 = ::std::mem::size_of::<f64>() as u32;
    pub const Int: u32 = ::std::mem::size_of::<i32>() as u32;
    pub const Long: u32 = ::std::mem::size_of::<i64>() as u32;
    pub const Bool: u32 = ::std::mem::size_of::<bool>() as u32;
}
