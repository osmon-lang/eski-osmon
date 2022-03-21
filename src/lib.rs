#![warn(rust_2018_idioms)]
#[allow(deprecated)]
#[allow(mutable_borrow_reservation_conflict)]

pub mod builtins;
pub mod class;
pub mod code;
pub mod compiler;
pub mod parser;
pub mod std_library;
pub use self::compiler::Compiler;
