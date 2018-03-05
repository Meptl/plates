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
