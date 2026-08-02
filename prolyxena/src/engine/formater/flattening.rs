use indexmap::IndexMap;

use crate::engine::core::*;

pub trait Flattening {
    fn expand(&mut self);
    fn flatten(&mut self);
}

impl Flattening for NixValue {
    fn expand(&mut self) {
        match self {
            NixValue::AttrSet(map) => {
                let mut new_map: IndexMap<String, NixValue> = IndexMap::new();
                let old_map = std::mem::take(map);
                for (key, mut value) in old_map {
                    value.expand();

                    let mut parts = Vec::new();
                    let mut current_part = String::new();
                    let mut in_quotes = false;
                    let mut depth = 0;
                    let mut chars = key.chars().peekable();

                    while let Some(c) = chars.next() {
                        match c {
                            '"' => {
                                in_quotes = !in_quotes;
                                current_part.push(c);
                            }
                            '$' => {
                                current_part.push(c);
                                if let Some(&'{') = chars.peek() {
                                    current_part.push('{');
                                    chars.next();
                                    depth += 1;
                                }
                            }
                            '{' if depth > 0 => {
                                depth += 1;
                                current_part.push(c);
                            }
                            '}' if depth > 0 => {
                                depth -= 1;
                                current_part.push(c);
                            }
                            '.' if !in_quotes && depth == 0 => {
                                parts.push(current_part.clone());
                                current_part.clear();
                            }
                            _ => {
                                current_part.push(c);
                            }
                        }
                    }

                    if !current_part.is_empty() {
                        parts.push(current_part);
                    }

                    let mut cur_level = &mut new_map;

                    for (i, part) in parts.iter().enumerate() {
                        if i == parts.len() - 1 {
                            cur_level.insert(part.to_string(), value.clone());
                        } else {
                            let node =
                                cur_level.entry(part.to_string()).or_insert_with(|| NixValue::AttrSet(IndexMap::new()));

                            if let NixValue::AttrSet(inner_map) = node {
                                cur_level = inner_map;
                            } else {
                                *node = NixValue::AttrSet(IndexMap::new());
                                if let NixValue::AttrSet(inner_map) = node {
                                    cur_level = inner_map;
                                } else {
                                    unreachable!()
                                }
                            }
                        }
                    }
                }
                *map = new_map;
            }
            NixValue::LetIn(map, body) => {
                let mut new_map: IndexMap<String, NixValue> = IndexMap::new();
                let old_map = std::mem::take(map);
                for (key, mut value) in old_map {
                    value.expand();

                    let mut parts = Vec::new();
                    let mut current_part = String::new();
                    let mut in_quotes = false;
                    let mut depth = 0;
                    let mut chars = key.chars().peekable();

                    while let Some(c) = chars.next() {
                        match c {
                            '"' => {
                                in_quotes = !in_quotes;
                                current_part.push(c);
                            }
                            '$' => {
                                current_part.push(c);
                                if let Some(&'{') = chars.peek() {
                                    current_part.push('{');
                                    chars.next();
                                    depth += 1;
                                }
                            }
                            '{' if depth > 0 => {
                                depth += 1;
                                current_part.push(c);
                            }
                            '}' if depth > 0 => {
                                depth -= 1;
                                current_part.push(c);
                            }
                            '.' if !in_quotes && depth == 0 => {
                                parts.push(current_part.clone());
                                current_part.clear();
                            }
                            _ => {
                                current_part.push(c);
                            }
                        }
                    }

                    if !current_part.is_empty() {
                        parts.push(current_part);
                    }

                    let mut cur_level = &mut new_map;

                    for (i, part) in parts.iter().enumerate() {
                        if i == parts.len() - 1 {
                            cur_level.insert(part.to_string(), value.clone());
                        } else {
                            let node =
                                cur_level.entry(part.to_string()).or_insert_with(|| NixValue::AttrSet(IndexMap::new()));

                            if let NixValue::AttrSet(inner_map) = node {
                                cur_level = inner_map;
                            } else {
                                *node = NixValue::AttrSet(IndexMap::new());
                                if let NixValue::AttrSet(inner_map) = node {
                                    cur_level = inner_map;
                                } else {
                                    unreachable!()
                                }
                            }
                        }
                    }
                }
                *map = new_map;
                body.expand();
            }
            NixValue::List(vec) => {
                for i in vec {
                    i.expand();
                }
            }
            NixValue::Group(inner) => {
                inner.expand();
            }
            NixValue::Antiquotation(inner) => {
                inner.expand();
            }
            NixValue::With(lb, rb) => {
                lb.expand();
                rb.expand();
            }
            NixValue::Apply(lb, rb) => {
                lb.expand();
                rb.expand();
            }
            NixValue::Lambda(_, _, body) => {
                body.expand();
            }
            NixValue::BinaryOp { left, operator: _, right } => {
                left.expand();
                right.expand();
            }
            NixValue::IndStr(frag) => {
                for i in frag {
                    if let StringFragment::Antiquotation(ast_box) = i {
                        ast_box.expand();
                    }
                }
            }
            _ => {}
        }
    }

    fn flatten(&mut self) {
        match self {
            NixValue::AttrSet(map) => {
                let mut changed = true;
                while changed {
                    changed = false;
                    let mut new_map = IndexMap::new();
                    let old_map = std::mem::take(map);

                    for (key, mut value) in old_map {
                        value.flatten();

                        if let NixValue::AttrSet(inner_map) = value {
                            changed = true;
                            for (inner_key, inner_value) in inner_map {
                                new_map.insert(format!("{}.{}", key, inner_key), inner_value);
                            }
                        } else {
                            new_map.insert(key, value);
                        }
                    }
                    *map = new_map;
                }
            }
            NixValue::LetIn(map, body) => {
                let mut changed = true;
                while changed {
                    changed = false;
                    let mut new_map = IndexMap::new();
                    let old_map = std::mem::take(map);

                    for (key, mut value) in old_map {
                        value.flatten();

                        if let NixValue::AttrSet(inner_map) = value {
                            changed = true;
                            for (inner_key, inner_value) in inner_map {
                                new_map.insert(format!("{}.{}", key, inner_key), inner_value);
                            }
                        } else {
                            new_map.insert(key, value);
                        }
                    }
                    *map = new_map;
                }
                body.flatten();
            }
            NixValue::List(vec) => {
                for i in vec {
                    i.flatten();
                }
            }
            NixValue::Group(inner) => {
                inner.flatten();
            }
            NixValue::Antiquotation(inner) => {
                inner.flatten();
            }
            NixValue::With(lb, rb) => {
                lb.flatten();
                rb.flatten();
            }
            NixValue::Apply(lb, rb) => {
                lb.flatten();
                rb.flatten();
            }
            NixValue::Lambda(_, _, body) => {
                body.flatten();
            }
            NixValue::BinaryOp { left, operator: _, right } => {
                left.flatten();
                right.flatten();
            }
            NixValue::IndStr(frag) => {
                for i in frag {
                    if let StringFragment::Antiquotation(ast_box) = i {
                        ast_box.flatten();
                    }
                }
            }
            _ => {}
        }
    }
}
