#![warn(clippy::all, rust_2018_idioms)]
use std::collections::HashMap;

mod app;
pub use app::BinaryTreeApp;

mod tree;

pub enum State {
    Default,
    Active,
    Visited,
    Fathomed,
    Infeasible,
}

pub type States = HashMap<String, State>;