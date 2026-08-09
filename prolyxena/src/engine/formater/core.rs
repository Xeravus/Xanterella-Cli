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
                out.push_str(&inner_indent);
                for i in vec {
                    match i {
                        StringFragment::Text(s) => out.push_str(s),
                        StringFragment::Antiquotation(s) => out.push_str(&s.format_nix(0)),
                    }
                }
                out.push('\'');
                out.push('\'');
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
            NixValue::Lambda(vec, alias, body) => {
                let mut out = String::new();
                match alias {
                    LambdaTypes::Nofix => {
                        if vec.is_empty() {
                            out.push_str("{ }:");
                            out
                        } else {
                            out.push_str("{\n");

                            for i in vec {
                                if i != "..." {
                                    out.push_str(&format!("{}{},\n", inner_indent, i));
                                }
                            }
                            out.push_str(&format!("{}...\n", inner_indent));
                            out.push_str("}: ");
                            out.push_str(&body.format_nix(depth));
                            out
                        }
                    },
                    LambdaTypes::Prefix(prefix_alias) => {
                        out.push_str(prefix_alias);
                        out.push_str(" @ ");
                        if vec.is_empty() {
                            out.push_str("{ }:");
                            out
                        } else {
                            out.push_str("{\n");

                            for i in vec {
                                if i != "..." {
                                    out.push_str(&format!("{}{},\n", inner_indent, i));
                                }
                            }

                            out.push_str(&format!("{}...\n", inner_indent));
                            out.push_str("}: ");
                            out.push_str(&body.format_nix(depth));
                            out
                        }
                    },
                    LambdaTypes::Suffix(suffix_alias) => {
                        if vec.is_empty() {
                            out.push_str("{ }");
                            out
                        } else {
                            out.push_str("{\n");

                            for i in vec {
                                if i != "..." {
                                    out.push_str(&format!("{}{},\n", inner_indent, i));
                                }
                            }
                            out.push_str(&format!("{}...\n", inner_indent));
                            out.push_str("} @ ");
                            out.push_str(suffix_alias);
                            out.push_str(" : ");
                            out.push_str(&body.format_nix(depth));
                            out
                        }
                    },
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
    use super::*;
    use indexmap::IndexMap;

    #[test]
    fn test_engine_formater_core_attr_set_empty() {
        let inital_content = NixValue::AttrSet(IndexMap::new());
        let expected_content = "{ }";

        assert_eq!(inital_content.format_nix(0), expected_content);
    }

    #[test]
    fn test_engine_formater_core_attr_set_normal() {
        let inital_content = NixValue::AttrSet(IndexMap::from([(String::from("a"), NixValue::Identifier(String::from("a")))]));
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
        let inital_content = NixValue::List(vec![NixValue::Identifier(String::from("a")), NixValue::Identifier(String::from("b"))]);
        let expected_content = "[\n  a\n  b\n]";

        assert_eq!(inital_content.format_nix(0), expected_content);
    }
}
