use indexmap::IndexMap;

use crate::engine::core::{Lexer, NixValue, StringFragment};

pub trait Sort {
    fn sort_ast(&mut self);
}

impl Sort for NixValue {
    fn sort_ast(&mut self) {
        match self {
            NixValue::AttrSet(map) => {
                map.sort_keys();
                for (_, value) in map.iter_mut() {
                    value.sort_ast();
                }
            }
            NixValue::Lambda(vec, _, body) => {
                vec.sort();
                body.sort_ast();
            }
            NixValue::LetIn(map, body) => {
                map.sort_keys();
                for (_, value) in map.iter_mut() {
                    value.sort_ast();
                }
                body.sort_ast();
            }
            NixValue::With(box1, box2) => {
                box1.sort_ast();
                box2.sort_ast();
            }
            NixValue::Apply(box1, box2) => {
                box1.sort_ast();
                box2.sort_ast();
            }
            NixValue::List(vec) => {
                // vec.sort();
                for value in vec.iter_mut() {
                    value.sort_ast();
                }
            }
            NixValue::IndStr(fragments) => {
                for fragment in fragments.iter_mut() {
                    if let StringFragment::Antiquotation(ast_box) = fragment {
                        ast_box.sort_ast();
                    }
                }
            }
            NixValue::Group(value) => {
                value.sort_ast();
            }
            NixValue::Antiquotation(value) => {
                value.sort_ast();
            }
            NixValue::BinaryOp { left, operator: _, right } => {
                left.sort_ast();
                right.sort_ast();
            }
            _ => (),
        }
    }
}

#[cfg(test)]
#[path = "core_test.rs"]
mod tests;
