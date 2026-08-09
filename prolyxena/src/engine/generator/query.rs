use crate::engine::core::*;

pub trait Query {
    fn query_exact_mut<'a>(&'a mut self, path: &[&str]) -> Vec<&'a mut NixValue>;
    fn query_exact_inner<'a>(&'a mut self, path: &[&str], result: &mut Vec<&'a mut NixValue>);

    fn query_fuzzy_mut<'a>(&'a mut self, term: &str) -> Vec<&'a mut NixValue>;
    fn query_fuzzy_inner<'a>(&'a mut self, term: &str, result: &mut Vec<&'a mut NixValue>);
}

impl Query for NixValue {
    fn query_exact_mut<'a>(&'a mut self, path: &[&str]) -> Vec<&'a mut NixValue> {
        let mut result = Vec::new();
        self.query_exact_inner(path, &mut result);
        result
    }

    fn query_exact_inner<'a>(&'a mut self, path: &[&str], result: &mut Vec<&'a mut NixValue>) {
        if path.is_empty() {
            result.push(self);
            return;
        }

        match self {
            NixValue::AttrSet(map) => {
                if let Some(value) = map.get_mut(path[0]) {
                    value.query_exact_inner(&path[1..], result);
                }
            }
            NixValue::LetIn(map, body) => {
                if let Some(value) = map.get_mut(path[0]) {
                    value.query_exact_inner(&path[1..], result);
                }
                body.query_exact_inner(path, result);
            }
            NixValue::List(vec) => {
                for i in vec.iter_mut() {
                    i.query_exact_inner(path, result);
                }
            }
            NixValue::Apply(_, right) | NixValue::Group(right) => {
                right.query_exact_inner(path, result);
            }
            _ => {}
        }
    }

    fn query_fuzzy_mut<'a>(&'a mut self, term: &str) -> Vec<&'a mut NixValue> {
        let mut result = Vec::new();
        let term_lower = term.to_lowercase();
        self.query_fuzzy_inner(&term_lower, &mut result);
        result
    }

    fn query_fuzzy_inner<'a>(&'a mut self, term: &str, result: &mut Vec<&'a mut NixValue>) {
        let is_self_match = match self {
            NixValue::Identifier(id) => id.to_lowercase().contains(term),
            _ => false,
        };

        if is_self_match {
            result.push(self);
            return;
        }

        match self {
            NixValue::AttrSet(map) => {
                for (key, value) in map.iter_mut() {
                    if key.to_lowercase().contains(term) {
                        result.push(value);
                    } else {
                        value.query_fuzzy_inner(term, result);
                    }
                }
            }
            NixValue::LetIn(map, body) => {
                for (key, value) in map.iter_mut() {
                    if key.to_lowercase().contains(term) {
                        result.push(value);
                    } else {
                        value.query_fuzzy_inner(term, result);
                    }
                }
                body.query_fuzzy_inner(term, result);
            }
            NixValue::List(vec) => {
                for i in vec.iter_mut() {
                    i.query_fuzzy_inner(term, result);
                }
            }
            NixValue::Group(inner) | NixValue::Antiquotation(inner) => {
                inner.query_fuzzy_inner(term, result);
            }
            NixValue::Apply(left, right) | NixValue::With(left, right) | NixValue::BinaryOp {left, right, ..} => {
                left.query_fuzzy_inner(term, result);
                right.query_fuzzy_inner(term, result);
            }
            _ => {}
        }
    }
}
