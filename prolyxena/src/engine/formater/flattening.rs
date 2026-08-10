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

#[cfg(test)]
mod tests {
    use crate::engine::lexer::core::*;
    use super::*;
    
    fn expand_assert(ini: &str, exp: &str) {
        let mut lexer_ini = Lexer::new(ini, String::from("path.nix"));
        let mut lexer_exp = Lexer::new(exp, String::from("path.nix"));

        let mut result_ini = lexer_ini.parse_value().unwrap();
        let result_exp = lexer_exp.parse_value().unwrap();

        result_ini.expand();

        assert_eq!(result_ini, result_exp);
    }

    fn flatten_assert(ini: &str, exp: &str) {
        let mut lexer_ini = Lexer::new(ini, String::from("path.nix"));
        let mut lexer_exp = Lexer::new(exp, String::from("path.nix"));

        let mut result_ini = lexer_ini.parse_value().unwrap();
        let result_exp = lexer_exp.parse_value().unwrap();

        result_ini.flatten();

        assert_eq!(result_ini, result_exp);
    }

    #[test]
    fn test_engine_formater_flattening_expand_attr_set_normal() {
        let initial_content = "{ a.b.c = true; }";
        let expectet_content = "{ a = { b = { c = true; }; }; }";
        expand_assert(initial_content, expectet_content);
    }
    
    #[test]
    fn test_engine_formater_flattening_expand_attr_set_quotes() {
        let initial_content = "{ a.\"b\".c = true; }";
        let expectet_content = "{ a = { \"b\" = { c = true; }; }; }";
        expand_assert(initial_content, expectet_content);
    }
    
    #[test]
    fn test_engine_formater_flattening_expand_attr_set_antiquotation() {
        let initial_content = "{ a.\"${b}\".c = true; }";
        let expectet_content = "{ a = { \"${b}\" = { c = true; }; }; }";
        expand_assert(initial_content, expectet_content);
    }
    
    #[test]
    fn test_engine_formater_flattening_expand_attr_set_nothing_to_expand() {
        let initial_content = "{ a = true; }";
        let expectet_content = "{ a = true; }";
        expand_assert(initial_content, expectet_content);
    }
    
    #[test]
    fn test_engine_formater_flattening_expand_let_in_no_body_normal() {
        let initial_content = "let a.b.c = true; in {}";
        let expectet_content = "let a = { b = { c = true; }; }; in {}";
        expand_assert(initial_content, expectet_content);
    }
    
    #[test]
    fn test_engine_formater_flattening_expand_let_in_no_body_quotes() {
        let initial_content = "let a.\"b\".c = true; in {}";
        let expectet_content = "let a = { \"b\" = { c = true; }; }; in {}";
        expand_assert(initial_content, expectet_content);
    }
    
    #[test]
    fn test_engine_formater_flattening_expand_let_in_no_body_antiquotation() {
        let initial_content = "let a.\"${b}\".c = true; in {}";
        let expectet_content = "let a = { \"${b}\" = { c = true; }; }; in {}";
        expand_assert(initial_content, expectet_content);
    }
    
    #[test]
    fn test_engine_formater_flattening_expand_let_in_body_normal() {
        let initial_content = "let a.b.c = true; in { a.b.c = true; }";
        let expectet_content = "let a = { b = { c = true; }; }; in { a = { b = { c = true; }; }; }";
        expand_assert(initial_content, expectet_content);
    }
    
    #[test]
    fn test_engine_formater_flattening_expand_let_in_body_quotes() {
        let initial_content = "let a.\"b\".c = true; in { a.\"b\".c = true; }";
        let expectet_content = "let a = { \"b\" = { c = true; }; }; in { a = { \"b\" = { c = true; }; }; }";
        expand_assert(initial_content, expectet_content);
    }
    
    #[test]
    fn test_engine_formater_flattening_expand_let_in_body_antiquotation() {
        let initial_content = "let a.\"${b}\".c = true; in { a.\"${b}\".c = true; }";
        let expectet_content = "let a = { \"${b}\" = { c = true; }; }; in { a = { \"${b}\" = { c = true; }; }; }";
        expand_assert(initial_content, expectet_content);
    }

    #[test]
    fn test_engine_formater_flattening_expand_list_normal() {
        let initial_content = "{ a = [ a b c ]; }";
        let expectet_content = "{ a = [ a b c ]; }";
        expand_assert(initial_content, expectet_content);
    }

    #[test]
    fn test_engine_formater_flattening_expand_group() {
        let initial_content = "{ a = (b { c.d.e = true; }); }";
        let expectet_content = "{ a = (b { c = { d = { e = true; }; }; }); }";
        expand_assert(initial_content, expectet_content);
    }

    #[test]
    fn test_engine_formater_flattening_expand_antiquotation() {
        let initial_content = "{ a = \"${pkgs.pkgs.pkgs}\"; }";
        let expectet_content = "{ a = \"${pkgs.pkgs.pkgs}\"; }";
        expand_assert(initial_content, expectet_content);
    }

    #[test]
    fn test_engine_formater_flattening_expand_with() {
        let initial_content = "{ a = with pkgs.pkgs; []; }";
        let expectet_content = "{ a = with pkgs.pkgs; []; }";
        expand_assert(initial_content, expectet_content);
    }

    #[test]
    fn test_engine_formater_flattening_expand_apply() {
        let initial_content = "{ a = (a b{ a.b.c = true; }); }";
        let expectet_content = "{ a = (a b{ a = { b = { c = true; }; }; }); }";
        expand_assert(initial_content, expectet_content);
    }

    #[test]
    fn test_engine_formater_flattening_expand_lambda() {
        let initial_content = "{}: { a.b.c = true; }";
        let expectet_content = "{ }: { a = { b = { c = true; }; }; }";
        expand_assert(initial_content, expectet_content);
    }

    #[test]
    fn test_engine_formater_flattening_expand_binary_op() {
        let initial_content = "{ a = b.c.d ++ e.f.g; }";
        let expectet_content = "{ a = b.c.d ++ e.f.g; }";
        expand_assert(initial_content, expectet_content);
    }
    #[test]
    fn test_engine_formater_flattening_expand_ind_str_antiquotation() {
        let initial_content = "{ a = '' ${pkgs.pkgs.pkgs} ''; }";
        let expectet_content = "{ a = '' ${pkgs.pkgs.pkgs} ''; }";
        expand_assert(initial_content, expectet_content);
    }

    #[test]
    fn test_engine_formater_flattening_flatten_attr_set_normal() {
        let expectet_content = "{ a.b.c = true; }";
        let initial_content = "{ a = { b = { c = true; }; }; }";
        flatten_assert(initial_content, expectet_content);
    }
    
    #[test]
    fn test_engine_formater_flattening_flatten_attr_set_quotes() {
        let expectet_content = "{ a.\"b\".c = true; }";
        let initial_content = "{ a = { \"b\" = { c = true; }; }; }";
        flatten_assert(initial_content, expectet_content);
    }
    
    #[test]
    fn test_engine_formater_flattening_flatten_attr_set_antiquotation() {
        let expectet_content = "{ a.\"${b}\".c = true; }";
        let initial_content = "{ a = { \"${b}\" = { c = true; }; }; }";
        flatten_assert(initial_content, expectet_content);
    }
    
    #[test]
    fn test_engine_formater_flattening_flatten_attr_set_nothing_to_flatten() {
        let expectet_content = "{ a = true; }";
        let initial_content = "{ a = true; }";
        flatten_assert(initial_content, expectet_content);
    }
    
    #[test]
    fn test_engine_formater_flattening_flatten_let_in_no_body_normal() {
        let expectet_content = "let a.b.c = true; in {}";
        let initial_content = "let a = { b = { c = true; }; }; in {}";
        flatten_assert(initial_content, expectet_content);
    }
    
    #[test]
    fn test_engine_formater_flattening_flatten_let_in_no_body_quotes() {
        let expectet_content = "let a.\"b\".c = true; in {}";
        let initial_content = "let a = { \"b\" = { c = true; }; }; in {}";
        flatten_assert(initial_content, expectet_content);
    }
    
    #[test]
    fn test_engine_formater_flattening_flatten_let_in_no_body_antiquotation() {
        let expectet_content = "let a.\"${b}\".c = true; in {}";
        let initial_content = "let a = { \"${b}\" = { c = true; }; }; in {}";
        flatten_assert(initial_content, expectet_content);
    }
    
    #[test]
    fn test_engine_formater_flattening_flatten_let_in_body_normal() {
        let expectet_content = "let a.b.c = true; in { a.b.c = true; }";
        let initial_content = "let a = { b = { c = true; }; }; in { a = { b = { c = true; }; }; }";
        flatten_assert(initial_content, expectet_content);
    }
    
    #[test]
    fn test_engine_formater_flattening_flatten_let_in_body_quotes() {
        let expectet_content = "let a.\"b\".c = true; in { a.\"b\".c = true; }";
        let initial_content = "let a = { \"b\" = { c = true; }; }; in { a = { \"b\" = { c = true; }; }; }";
        flatten_assert(initial_content, expectet_content);
    }
    
    #[test]
    fn test_engine_formater_flattening_flatten_let_in_body_antiquotation() {
        let expectet_content = "let a.\"${b}\".c = true; in { a.\"${b}\".c = true; }";
        let initial_content = "let a = { \"${b}\" = { c = true; }; }; in { a = { \"${b}\" = { c = true; }; }; }";
        flatten_assert(initial_content, expectet_content);
    }

    #[test]
    fn test_engine_formater_flattening_flatten_list_normal() {
        let expectet_content = "{ a = [ a b c ]; }";
        let initial_content = "{ a = [ a b c ]; }";
        flatten_assert(initial_content, expectet_content);
    }

    #[test]
    fn test_engine_formater_flattening_flatten_group() {
        let expectet_content = "{ a = (b { c.d.e = true; }); }";
        let initial_content = "{ a = (b { c = { d = { e = true; }; }; }); }";
        flatten_assert(initial_content, expectet_content);
    }

    #[test]
    fn test_engine_formater_flattening_flatten_antiquotation() {
        let expectet_content = "{ a = \"${pkgs.pkgs.pkgs}\"; }";
        let initial_content = "{ a = \"${pkgs.pkgs.pkgs}\"; }";
        flatten_assert(initial_content, expectet_content);
    }

    #[test]
    fn test_engine_formater_flattening_flatten_with() {
        let expectet_content = "{ a = with pkgs.pkgs; []; }";
        let initial_content = "{ a = with pkgs.pkgs; []; }";
        flatten_assert(initial_content, expectet_content);
    }

    #[test]
    fn test_engine_formater_flattening_flatten_apply() {
        let expectet_content = "{ a = (a b{ a.b.c = true; }); }";
        let initial_content = "{ a = (a b{ a = { b = { c = true; }; }; }); }";
        flatten_assert(initial_content, expectet_content);
    }

    #[test]
    fn test_engine_formater_flattening_flatten_lambda() {
        let expectet_content = "{ }: { a.b.c = true; }";
        let initial_content = "{}: { a = { b = { c = true; }; }; }";
        flatten_assert(initial_content, expectet_content);
    }

    #[test]
    fn test_engine_formater_flattening_flatten_binary_op() {
        let expectet_content = "{ a = b.c.d ++ e.f.g; }";
        let initial_content = "{ a = b.c.d ++ e.f.g; }";
        flatten_assert(initial_content, expectet_content);
    }

    #[test]
    fn test_engine_formater_flattening_flatten_ind_str_antiquotation() {
        let expectet_content = "{ a = '' ${pkgs.pkgs.pkgs} ''; }";
        let initial_content = "{ a = '' ${pkgs.pkgs.pkgs} ''; }";
        flatten_assert(initial_content, expectet_content);
    }
}
