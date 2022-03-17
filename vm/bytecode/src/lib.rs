#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
extern crate osmon_vm;


pub mod opcode;
pub mod parser;
pub mod assembler;

#[macro_export]
macro_rules! encode {
    ($v:expr; $t: ty) => {
        unsafe {
            ::std::mem::transmute::<$t,[u8;::std::mem::size_of::<$t>()]>($v)
        }
    };
}

#[macro_export]
macro_rules! decode {
    ($arr: expr; $t: ty) => {
        unsafe {
            ::std::mem::transmute::<[u8;::std::mem::size_of::<$t>()],$t>($arr)
        }
    };
}
