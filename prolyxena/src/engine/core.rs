use std::collections::HashMap;
use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, Clone)]
pub enum NixValue {
    AttrSet(HashMap<String, NixValue>),
    List(Vec<NixValue>),
    Str(String),
    Int(u64),
    Float(f64),
    Bool(bool),
    Identifier(String),
    Group(Box<NixValue>),
    LetIn(HashMap<String, NixValue>, Box<NixValue>),
    With(Box<NixValue>, Box<NixValue>),
    Lambda(Vec<String>, Option<String>, Box<NixValue>),
    Apply(Box<NixValue>, Box<NixValue>),
    Path(String),
    BinaryOp {
        left: Box<NixValue>,
        operator: Operator,
        right: Box<NixValue>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    Add,
    Sub,
    Concat,
    Equal,
    Merge,
    Divide,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParseEvent {
    // Start
    StartAttrSet,
    StartList,
    StartLetIn,
    StartLambda,
    StartWith,
    StartString,
    StartPath,
    StartNumber,
    StartExpression,
    StartOperator,
    StartIdentifier,
    StartWhitespace,
    StartValue,
    StartGroup,
    // End
    EndAttrSet,
    EndList,
    EndLetIn,
    EndLambda,
    EndWith,
    EndString,
    EndPath,
    EndNumber,
    EndExpression,
    EndOperator,
    EndIdentifier,
    EndWhitespace,
    EndValue,
    EndGroup,
}

pub struct Lexer<'a> {
    pub chars: Peekable<Chars<'a>>,
    pub event: Vec<ParseEvent>,
    pub path: String,
}

impl<'a> Lexer<'a> {
    pub fn new(content: &'a str, path: String) -> Self {
        Lexer {
            chars: content.chars().peekable(),
            event: vec![],
            path,
        }
    }
}

#[cfg(test)]
#[path = "core_test.rs"]
mod tests;
