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
            },
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
            },
            NixValue::Int(i) => i.to_string(),
            NixValue::Float(i) => i.to_string(),
            NixValue::Bool(b) => if *b { 
                "true".to_string()
            } else {
                "false".to_string()
            },
            NixValue::Identifier(i) => i.clone(),
            NixValue::Group(b) => {
                let mut out = String::from("(");
                out.push_str(&b.format_nix(depth));
                out.push(')');
                out
            },
            NixValue::LetIn(map, b) => {
                let mut out = String::from("let\n");
                for (key, value) in map {
                    let formated = value.format_nix(depth + 1);
                    out.push_str(&format!("{}{} = {};\n", inner_indent, key, formated));
                }
                out.push_str(&format!("{}in ", indent));
                out.push_str(&b.format_nix(depth + 1));
                out
            },
            NixValue::With(a, b) => {
                let mut out = String::from("with ");
                out.push_str(&a.format_nix(0));
                out.push(';');
                out.push(' ');
                out.push_str(&b.format_nix(depth));
                out
            },
            NixValue::Lambda(vec, alias, b) => {
                let mut out = String::new();
                if vec.is_empty() {
                    out.push_str("{ }:");
                } else {
                    out.push_str("{\n");
                }
                
                for i in vec {
                    if i != "..." {
                        out.push_str(&format!("{}{},\n", inner_indent, i));
                    }
                }
                out.push_str(&format!("{}...\n", inner_indent));
                out.push('}');

                if let Some(al) = alias {
                    out.push_str(&format!(" @ {}", al));
                }
                out.push(':');
                out.push(' ');
                out.push_str(&b.format_nix(depth));
                out
            },
            NixValue::Apply(lb, rb) => {
                let mut out = String::new();
                out.push_str(&lb.format_nix(depth + 1));
                out.push(' ');
                out.push_str(&rb.format_nix(depth));
                out
            },
            NixValue::Path(s) => s.clone(),
            NixValue::Antiquotation(b) => {
                let mut out = String::from("${");
                out.push_str(&b.format_nix(0));
                out.push('}');
                out
            },
            NixValue::BinaryOp { left: lb, operator: op, right: rb } => {
                let mut out = String::new();
                out.push_str(&lb.format_nix(0));

                match op {
                    Operator::Add => {
                        out.push_str(" + ");
                    },
                    Operator::Sub => {
                        out.push_str(" - ");
                    },
                    Operator::Concat => {
                        out.push_str(" ++ ");
                    },
                    Operator::Equal => {
                        out.push_str(" == ");
                    },
                    Operator::Merge => {
                        out.push_str(" // ");
                    },
                    Operator::Divide => {
                        out.push_str(" / ");
                    },
                }

                out.push_str(&rb.format_nix(depth));
                out
            },
        }
    }
}
