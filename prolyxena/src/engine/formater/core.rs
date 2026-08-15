use crate::engine::core::*;

pub trait Format {
    fn format_nix(&self, depth: usize) -> String;
}

impl Format for NixValue {
    fn format_nix(&self, depth: usize) -> String {
        let indent = "  ".repeat(depth);
        let inner_indent = "  ".repeat(depth + 1);

        match self {
            NixValue::AttrSet(map) => {
                if map.is_empty() {
                    return "{ }".to_string();
                }

                let mut out = String::from("{\n");
                for (key, value) in map {
                    let formated = value.format_nix(depth + 1);
                    out.push_str(&format!("{}{} = {};\n", inner_indent, key, formated));
                }
                out.push_str(&format!("{}}}", indent));
                out
            }
            NixValue::List(vec) => {
                if vec.is_empty() {
                    return "[ ]".to_string();
                }

                let mut out = String::from("[\n");
                for i in vec {
                    let formated = i.format_nix(depth + 1);
                    out.push_str(&format!("{}{}\n", inner_indent, formated));
                }
                out.push_str(&format!("{}]", indent));
                out
            }
            NixValue::Str(s) => format!("\"{}\"", s),
            NixValue::IndStr(vec) => {
                if vec.is_empty() {
                    return "'' ''".to_string();
                }

                let mut out = String::from("''");
                for i in vec {
                    match i {
                        StringFragment::Text(s) => out.push_str(s),
                        StringFragment::Antiquotation(s) => {
                            out.push('$');
                            out.push('{');
                            out.push_str(&s.format_nix(0));
                            out.push('}');
                        }
                    }
                }
                out.push_str("''");
                out
            }
            NixValue::Int(i) => i.to_string(),
            NixValue::Float(i) => i.to_string(),
            NixValue::Bool(b) => {
                if *b {
                    "true".to_string()
                } else {
                    "false".to_string()
                }
            }
            NixValue::Identifier(i) => i.clone(),
            NixValue::Group(b) => {
                let mut out = String::from("(");
                out.push_str(&b.format_nix(depth));
                out.push(')');
                out
            }
            NixValue::LetIn(map, b) => {
                let mut out = String::from("let\n");
                for (key, value) in map {
                    let formated = value.format_nix(depth + 1);
                    out.push_str(&format!("{}{} = {};\n", inner_indent, key, formated));
                }
                out.push_str(&format!("{}in ", indent));
                out.push_str(&b.format_nix(depth + 1));
                out
            }
            NixValue::With(a, b) => {
                let mut out = String::from("with ");
                out.push_str(&a.format_nix(0));
                out.push(';');
                out.push(' ');
                out.push_str(&b.format_nix(depth));
                out
            }
            NixValue::Lambda(lambdatype) => {
                let mut out = String::new();
                match lambdatype {
                    LambdaTypes::Nofix(vec, body) => {
                        if vec.is_empty() {
                            out.push_str("{ }:");
                            out
                        } else {
                            out.push_str("{\n");

                            for i in vec {
                                if i != "..." {
                                    out.push_str(&format!("{}{},\n", inner_indent, i))
                                }
                            }
                            out.push_str(&format!("{}...\n", inner_indent));
                            out.push_str(&format!("{}}}: ", indent));
                            out.push_str(&body.format_nix(depth));
                            out
                        }
                    }
                    LambdaTypes::Suffix(vec, alias, body) => {
                        if vec.is_empty() {
                            out.push_str("{ }: ");
                        } else {
                            out.push_str("{\n");

                            for i in vec {
                                if i != "..." {
                                    out.push_str(&format!("{}{},\n", inner_indent, i));
                                }
                            }
                            out.push_str(&format!("{}...\n", inner_indent));
                            out.push_str(&format!("{}}} ", indent));
                        }
                        out.push_str(&format!("@ {} : ", alias));
                        out.push_str(&body.format_nix(depth));
                        out
                    }
                    LambdaTypes::Prefix(vec, alias, body) => {
                        out.push_str(&format!("{} @ ", alias));
                        if vec.is_empty() {
                            out.push_str(" { }: ");
                            out
                        } else {
                            out.push_str("{\n");

                            for i in vec {
                                if i != "..." {
                                    out.push_str(&format!("{}{},\n", inner_indent, i))
                                }
                            }
                            out.push_str(&format!("{}...\n", inner_indent));
                            out.push_str(&format!("{}}}: ", indent));
                            out.push_str(&body.format_nix(depth));
                            out
                        }
                    }
                    LambdaTypes::Single(alias, body) => {
                        out.push_str(&format!("{}: ", alias));
                        out.push_str(&body.format_nix(depth));
                        out
                    }
                }
            }
            NixValue::Apply(lb, rb) => {
                let mut out = String::new();
                out.push_str(&lb.format_nix(depth + 1));
                out.push(' ');
                out.push_str(&rb.format_nix(depth));
                out
            }
            NixValue::Path(s) => s.clone(),
            NixValue::Antiquotation(b) => {
                let mut out = String::from("${");
                out.push_str(&b.format_nix(0));
                out.push('}');
                out
            }
            NixValue::BinaryOp { left: lb, operator: op, right: rb } => {
                let mut out = String::new();
                out.push_str(&lb.format_nix(0));

                match op {
                    Operator::Add => {
                        out.push_str(" + ");
                    }
                    Operator::Sub => {
                        out.push_str(" - ");
                    }
                    Operator::Concat => {
                        out.push_str(" ++ ");
                    }
                    Operator::Equal => {
                        out.push_str(" == ");
                    }
                    Operator::Unequal => {
                        out.push_str(" != ");
                    }
                    Operator::Merge => {
                        out.push_str(" // ");
                    }
                    Operator::Divide => {
                        out.push_str(" / ");
                    }
                }

                out.push_str(&rb.format_nix(depth));
                out
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use indexmap::IndexMap;

    use super::*;

    #[test]
    fn test_engine_formater_core_attr_set_empty() {
        let inital_content = NixValue::AttrSet(IndexMap::new());
        let expected_content = "{ }";

        assert_eq!(inital_content.format_nix(0), expected_content);
    }

    #[test]
    fn test_engine_formater_core_attr_set_normal() {
        let inital_content =
            NixValue::AttrSet(IndexMap::from([(String::from("a"), NixValue::Identifier(String::from("a")))]));
        let expected_content = "{\n  a = a;\n}";

        assert_eq!(inital_content.format_nix(0), expected_content);
    }

    #[test]
    fn test_engine_formater_core_list_empty() {
        let inital_content = NixValue::List(Vec::new());
        let expected_content = "[ ]";

        assert_eq!(inital_content.format_nix(0), expected_content);
    }

    #[test]
    fn test_engine_formater_core_list_normal() {
        let inital_content =
            NixValue::List(vec![NixValue::Identifier(String::from("a")), NixValue::Identifier(String::from("b"))]);
        let expected_content = "[\n  a\n  b\n]";

        assert_eq!(inital_content.format_nix(0), expected_content);
    }

    #[test]
    fn test_engine_formater_core_str() {
        let initial_content = NixValue::Str(String::from("hello world"));
        let expected_content = "\"hello world\"";

        assert_eq!(initial_content.format_nix(0), expected_content);
    }

    #[test]
    fn test_engine_formater_core_ind_str_empty() {
        let initial_content = NixValue::IndStr(Vec::new());
        let expected_content = "'' ''";

        assert_eq!(initial_content.format_nix(0), expected_content);
    }

    #[test]
    fn test_engine_formater_core_ind_str_normal() {
        let initial_content = NixValue::IndStr(vec![
            StringFragment::Text(String::from("line 1\n")),
            StringFragment::Text(String::from("line 2\n")),
        ]);
        let expected_content = "''line 1\nline 2\n''";

        assert_eq!(initial_content.format_nix(0), expected_content);
    }

    #[test]
    fn test_engine_formater_core_ind_str_antiquotation() {
        let initial_content = NixValue::IndStr(vec![
            StringFragment::Text(String::from("value is ")),
            StringFragment::Antiquotation(Box::new(NixValue::Identifier(String::from("var")))),
            StringFragment::Text(String::from("\n")),
        ]);
        let expected_content = "''value is ${var}\n''";

        assert_eq!(initial_content.format_nix(0), expected_content);
    }

    // --- Primitive Tests ---
    #[test]
    fn test_engine_formater_core_int() {
        let initial_content = NixValue::Int(42);
        let expected_content = "42";

        assert_eq!(initial_content.format_nix(0), expected_content);
    }

    #[test]
    fn test_engine_formater_core_float() {
        let initial_content = NixValue::Float(3.14);
        let expected_content = "3.14";

        assert_eq!(initial_content.format_nix(0), expected_content);
    }

    #[test]
    fn test_engine_formater_core_bool_true() {
        let initial_content = NixValue::Bool(true);
        let expected_content = "true";

        assert_eq!(initial_content.format_nix(0), expected_content);
    }

    #[test]
    fn test_engine_formater_core_bool_false() {
        let initial_content = NixValue::Bool(false);
        let expected_content = "false";

        assert_eq!(initial_content.format_nix(0), expected_content);
    }

    #[test]
    fn test_engine_formater_core_identifier() {
        let initial_content = NixValue::Identifier(String::from("myVar"));
        let expected_content = "myVar";

        assert_eq!(initial_content.format_nix(0), expected_content);
    }

    #[test]
    fn test_engine_formater_core_path() {
        let initial_content = NixValue::Path(String::from("./config.nix"));
        let expected_content = "./config.nix";

        assert_eq!(initial_content.format_nix(0), expected_content);
    }

    // --- Struktur-Tests ---
    #[test]
    fn test_engine_formater_core_group() {
        let initial_content = NixValue::Group(Box::new(NixValue::Identifier(String::from("inner"))));
        let expected_content = "(inner)";

        assert_eq!(initial_content.format_nix(0), expected_content);
    }

    #[test]
    fn test_engine_formater_core_let_in() {
        let initial_content = NixValue::LetIn(
            IndexMap::from([(String::from("a"), NixValue::Int(1))]),
            Box::new(NixValue::Identifier(String::from("a"))),
        );
        let expected_content = "let\n  a = 1;\nin a";

        assert_eq!(initial_content.format_nix(0), expected_content);
    }

    #[test]
    fn test_engine_formater_core_with() {
        let initial_content = NixValue::With(
            Box::new(NixValue::Identifier(String::from("pkgs"))),
            Box::new(NixValue::List(vec![NixValue::Identifier(String::from("git"))])),
        );
        let expected_content = "with pkgs; [\n  git\n]";

        assert_eq!(initial_content.format_nix(0), expected_content);
    }

    #[test]
    fn test_engine_formater_core_antiquotation() {
        let initial_content = NixValue::Antiquotation(Box::new(NixValue::Identifier(String::from("var"))));
        let expected_content = "${var}";

        assert_eq!(initial_content.format_nix(0), expected_content);
    }

    #[test]
    fn test_engine_formater_core_apply() {
        let initial_content = NixValue::Apply(
            Box::new(NixValue::Identifier(String::from("lib.mkIf"))),
            Box::new(NixValue::Identifier(String::from("true"))),
        );
        let expected_content = "lib.mkIf true";

        assert_eq!(initial_content.format_nix(0), expected_content);
    }

    // --- Lambda Tests ---
    #[test]
    fn test_engine_formater_core_lambda_nofix_empty() {
        let initial_content =
            NixValue::Lambda(LambdaTypes::Nofix(Vec::new(), Box::new(NixValue::AttrSet(IndexMap::new()))));
        let expected_content = "{ }:";

        assert_eq!(initial_content.format_nix(0), expected_content);
    }

    #[test]
    fn test_engine_formater_core_lambda_nofix_args() {
        let initial_content = NixValue::Lambda(LambdaTypes::Nofix(
            vec![String::from("pkgs"), String::from("lib")],
            Box::new(NixValue::AttrSet(IndexMap::new())),
        ));
        let expected_content = "{\n  pkgs,\n  lib,\n  ...\n}: { }";

        assert_eq!(initial_content.format_nix(0), expected_content);
    }

    #[test]
    fn test_engine_formater_core_lambda_single() {
        let initial_content =
            NixValue::Lambda(LambdaTypes::Single(String::from("config"), Box::new(NixValue::AttrSet(IndexMap::new()))));
        let expected_content = "config: { }";

        assert_eq!(initial_content.format_nix(0), expected_content);
    }

    #[test]
    fn test_engine_formater_core_lambda_suffix() {
        let initial_content = NixValue::Lambda(LambdaTypes::Suffix(
            vec![String::from("pkgs")],
            String::from("inputs"),
            Box::new(NixValue::AttrSet(IndexMap::new())),
        ));
        let expected_content = "{\n  pkgs,\n  ...\n} @ inputs : { }";

        assert_eq!(initial_content.format_nix(0), expected_content);
    }

    #[test]
    fn test_engine_formater_core_lambda_prefix() {
        let initial_content = NixValue::Lambda(LambdaTypes::Prefix(
            vec![String::from("pkgs")],
            String::from("inputs"),
            Box::new(NixValue::AttrSet(IndexMap::new())),
        ));
        let expected_content = "inputs @ {\n  pkgs,\n  ...\n}: { }";

        assert_eq!(initial_content.format_nix(0), expected_content);
    }

    // --- BinaryOp Tests ---
    #[test]
    fn test_engine_formater_core_binaryop_concat() {
        let initial_content = NixValue::BinaryOp {
            left: Box::new(NixValue::Identifier(String::from("list1"))),
            operator: Operator::Concat,
            right: Box::new(NixValue::Identifier(String::from("list2"))),
        };
        let expected_content = "list1 ++ list2";

        assert_eq!(initial_content.format_nix(0), expected_content);
    }

    #[test]
    fn test_engine_formater_core_binaryop_merge() {
        let initial_content = NixValue::BinaryOp {
            left: Box::new(NixValue::Identifier(String::from("set1"))),
            operator: Operator::Merge,
            right: Box::new(NixValue::Identifier(String::from("set2"))),
        };
        let expected_content = "set1 // set2";

        assert_eq!(initial_content.format_nix(0), expected_content);
    }
}
