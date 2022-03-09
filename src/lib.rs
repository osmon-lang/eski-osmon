#![warn(unused_must_use)]
#![warn(rust_2018_idioms)]
#![allow(clippy::redundant_closure)]
#![allow(clippy::vec_box)]
#![allow(dead_code)]
#![allow(unused_variables)]
#[macro_use]
pub mod macros;
pub mod syntax;

use std::sync::atomic::{AtomicUsize, Ordering};
pub use syntax::{
    ast,
    interner::{intern, str, INTERNER},
    position::Position,
};
static IDGEN: AtomicUsize = AtomicUsize::new(0);

// #[inline]
// pub fn gen_id() -> NodeId { NodeId(IDGEN.fetch_add(1, Ordering::AcqRel)) }

// use std::collections::HashMap;

pub struct Context
{
    // pub file: File,
    // pub types: HashMap<NodeId, Type>,
    pub opt: u8,
}