use indexmap::IndexMap;
use std::iter::Peekable;
use std::str::Chars;
use std::sync::mpsc::Sender;

#[derive(Debug, Clone, PartialEq)]
pub enum NixValue {
    AttrSet(IndexMap<String, NixValue>),
    List(Vec<NixValue>),
    Str(String),
    IndStr(Vec<StringFragment>),
    Int(u64),
    Float(f64),
    Bool(bool),
    Identifier(String),
    Group(Box<NixValue>),
    LetIn(IndexMap<String, NixValue>, Box<NixValue>),
    With(Box<NixValue>, Box<NixValue>),
    Lambda(Vec<String>, Option<String>, Box<NixValue>),
    Apply(Box<NixValue>, Box<NixValue>),
    Path(String),
    Antiquotation(Box<NixValue>),
    BinaryOp { left: Box<NixValue>, operator: Operator, right: Box<NixValue> },
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
pub enum StringFragment {
    Text(String),
    Antiquotation(Box<NixValue>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParseEvent {
    // Special
    Finished(String),
    StartGen,
    EndGen,
    StartGettingFiles,
    EndGettingFiles,
    StartParsingFile(String),
    EndParsingFile(String),
    StartSortingFile(String),
    EndSortingFile(String),
    // Error(Err),
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
    StartAntiquotation,
    StartIndentedString,
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
    EndAntiquotation,
    EndIndentedString,
}

pub struct Lexer<'a> {
    pub chars: Peekable<Chars<'a>>,
    pub path: String,
    pub trans: Option<Sender<ParseEvent>>,
}

impl<'a> Lexer<'a> {
    pub fn new(content: &'a str, path: String) -> Self {
        Lexer { chars: content.chars().peekable(), path, trans: None }
    }
    pub fn new_trans(content: &'a str, path: String, sender: Sender<ParseEvent>) -> Self {
        Lexer { chars: content.chars().peekable(), path, trans: Some(sender) }
    }

    pub fn log_event(&self, event: ParseEvent) {
        if let Some(tx) = &self.trans {
            let _ = tx.send(event);
        }
    }
}

#[cfg(test)]
#[path = "core_test.rs"]
mod tests;
