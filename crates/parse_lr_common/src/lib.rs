#![feature(gen_blocks)]

mod automaton;
mod table;
mod driver;

// LR 共通部品
pub use table::{LRAction, LRTable, LRTableBuilder};
pub use driver::LRDriver;

// LR オートマトン
pub use automaton::lr0;
pub use automaton::lr1;
