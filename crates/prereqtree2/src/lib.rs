#![allow(non_snake_case)]

use std::collections::HashMap;

#[macro_use]
mod macros;

#[derive(Debug, Clone)]
pub struct Node {
    code: String,
    sems: Vec<u8>,
    req: Tree2,
}

#[derive(Debug, Clone)]
pub enum Tree2 {
    None,
    Node(Box<Node>),
    And(Vec<Tree2>),
    Or(Vec<Tree2>),
}
use Tree2::*;

impl Tree2 {
    fn node(m: &Node) -> Self {
        Self::Node(Box::new(m.clone()))
    }
}

// {{{ shorteners
macro_rules! s {
    ($t:ident) => {
        stringify!($t).to_string()
    };
}
// }}}

pub fn main() {
}
