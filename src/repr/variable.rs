use ::itertools::Itertools;
use ::indextree::NodeId;
use ::std::iter::repeat;
use ::std::rc::Rc;
use super::prelude::*;

#[derive(PartialEq, Eq)]
pub enum VariableType {
    Integer,
    Float,
    Char,
    String, // TODO: Decide if this is a struct or not.
    Function,
    Struct
}

/// For now this will just catastrophicaly fail on invalid strings.
impl<'a> From<&'a str> for VariableType {
    fn from(s: &str) -> VariableType {
        match s.as_ref() {
            "int" | "integer" => VariableType::Integer,
            "float" | "double" => VariableType::Float,
            "char" => VariableType::Integer,
            "string" => VariableType::String,
            "fn" | "func" | "function" => VariableType::Function,
            "struct" => VariableType::Struct,
            _ => panic!("Unexpected value to convert to VariableType")
        }
    }
}

/// Variable information
pub struct Variable {
    pub name: VariableName,
    pub vartype: VariableType,
    pub description: Option<String>
}

impl Data for Variable {
}

/// Structure for code representation of variables
pub struct VariableName {
    /// Any string representation of the variable
    pub canonical: String,
}

impl VariableName {
    /// Returns the CamelCase representation of canonical. Reallocates
    pub fn camel_case(&self) -> String {
        let mut r = String::new();
        for token in self.tokens() {
            for (i, c) in token.chars().enumerate() {
                match i {
                    0 => r.extend(c.to_uppercase()),
                    _ => r.extend(c.to_lowercase()),
                }
            }
        }
        r
    }

    /// Returns the lowerCamelCase representation of canonical. Reallocates
    pub fn lower_camel_case(&self) -> String {
        let mut r = String::new();
        for (i, token) in self.tokens().enumerate() {
            for (j, c) in token.chars().enumerate() {
                match (i, j) {
                    (0, _) => r.extend(c.to_lowercase()),
                    (_, 0) => r.extend(c.to_uppercase()),
                    _ => r.extend(c.to_lowercase()),
                }
            }
        }
        r
    }

    /// Returns the snake_case representation of canonical. Reallocates
    pub fn snake_case(&self) -> String {
        self.tokens()
            .map(|s| s.to_lowercase())
            .intersperse(String::from("_"))
            .collect()
    }

    /// Returns an iterator for the tokens of canonical
    pub fn tokens<'a>(&'a self) -> impl Iterator<Item = &'a str> {
        self.canonical.split_whitespace()
                      .flat_map(|w| w.split(&['_', '-'][..]))
                      .flat_map(|w| w.chars().split_camel_case())
    }
}

use std::str::Chars;


pub trait SplitCamelCaseExt<'a>: Iterator + Sized {
    fn split_camel_case(self) -> SplitCamelCase<'a>;
}

impl<'a> SplitCamelCaseExt<'a> for Chars<'a> {
    fn split_camel_case(self) -> SplitCamelCase<'a> {
        let value = self.as_str().clone();
        SplitCamelCase { iter: self, value: value, start: 0, end: 0 }
    }
}

/// Iterate over the tokens of a camelcase.
pub struct SplitCamelCase<'a> {
    iter: Chars<'a>,
    value: &'a str,
    start: usize,
    end: usize,
}

impl<'a> Iterator for SplitCamelCase<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<&'a str> {
        // Check for end and assume a token size of atleast 1.
        if let None = self.iter.next() {
            return None;
        }

        self.start = self.end;
        let token_len = 1 + self.iter.clone().take_while(|c| !c.is_uppercase()).count();
        self.end += token_len;

        // token_len includes an extra char, so we pop one less.
        for _ in 1..token_len {
            self.iter.next().unwrap();
        }

        Some(&self.value[self.start..self.end])
    }
}

