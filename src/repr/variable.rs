use ::itertools::Itertools;
use ::indextree::NodeId;
use ::std::iter::repeat;
use ::std::rc::Rc;
use ::std::fmt;
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

impl VariableType {
    fn as_str(&self) -> &str {
        match (*self) {
            VariableType::Integer => "integer",
            VariableType::Float => "double",
            VariableType::Char => "char",
            VariableType::String => "string",
            VariableType::Function => "fn",
            VariableType::Struct => "struct",
        }
    }
}

/// For now this will just catastrophicaly fail on invalid strings.
impl ::std::str::FromStr for VariableType {
    //todo: Errortypes
    type Err = String;
    fn from_str(s: &str) -> Result<VariableType, Self::Err> {
        Ok(match s.as_ref() {
            "int" | "integer" => VariableType::Integer,
            "float" | "double" => VariableType::Float,
            "char" => VariableType::Integer,
            "string" => VariableType::String,
            "fn" | "func" | "function" => VariableType::Function,
            "struct" => VariableType::Struct,
            _ => panic!("Unexpected value to convert to VariableType")
        })
    }
}

impl fmt::Display for VariableType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.as_str().fmt(f)
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

/// Naively cache the first return value of the expression.
///
/// Note that the return value of the function is changed to a reference.
macro_rules! cached {
    (pub fn $name:ident (&$self:ident) -> $ret:ty $body:block) => {
        pub fn $name(&$self) -> &$ret {
            static mut cached: Option<$ret> = None;
            unsafe {
                if let Some(ref c) = cached {
                    return c;
                }
            }
            let val = $body;
            unsafe {
                cached = Some(val);
                cached.as_ref().unwrap()
            }
        }
    };
}

/// Various representations of a variable identifier.
///
/// Note that the use of the cached! macro is safe here because this structure is
/// immutable after initialization.
pub struct VariableName {
    /// Any string representation of the variable
    pub canonical: String
}

impl VariableName {
    /// Returns the CamelCase representation of canonical
    cached! {
        pub fn camel_case(&self) -> String {
            let mut output = String::new();
            for token in self.tokens() {
                for (i, c) in token.chars().enumerate() {
                    match i {
                        0 => output.extend(c.to_uppercase()),
                        _ => output.extend(c.to_lowercase()),
                    }
                }
            }
            output
        }
    }

    /// Returns the lowerCamelCase representation of canonical
    cached! {
        pub fn lower_camel_case(&self) -> String {
            let mut output = String::new();
            for (i, token) in self.tokens().enumerate() {
                for (j, c) in token.chars().enumerate() {
                    match (i, j) {
                        (0, _) => output.extend(c.to_lowercase()),
                        (_, 0) => output.extend(c.to_uppercase()),
                        _ => output.extend(c.to_lowercase()),
                    }
                }
            }
            output
        }
    }

    /// Returns the snake_case representation of canonical
    cached! {
        pub fn snake_case(&self) -> String {
            self.tokens()
                .map(|s| s.to_lowercase())
                .intersperse(String::from("_"))
                .collect()
        }
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

