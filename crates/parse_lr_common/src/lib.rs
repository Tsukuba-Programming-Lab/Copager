#![feature(gen_blocks)]

mod automaton;
mod table;
mod driver;
pub mod lr0;
pub mod lr1;

pub use table::{LRAction, LRTable, LRTableBuilder};
pub use driver::LRDriver;
