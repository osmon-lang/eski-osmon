#![warn(rust_2018_idioms)]
#![allow(non_snake_case)]

pub mod error;
pub mod frame;
pub mod function;
pub mod index;
pub mod jit;
pub mod machine;
pub mod object;
pub mod object_info;
pub mod object_pool;
pub mod opcodes;
pub mod static_root;
pub mod string;
pub mod value;

use time;

pub mod prelude {
    #[allow(unused_imports)]
    use super::*;
}
