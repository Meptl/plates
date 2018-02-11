/// Module for decoding specification representation of data into plates structures
/// Expects decoded Rust structures of a file. i.e. Values produced from serde-yaml.
use ::indextree::{Arena, NodeId};
use ::serde_yaml::Value;
use ::std::collections::HashMap;
use super::super::prelude::*;
use super::super::Program;
use super::super::variable::{ScalarType, Variable, VariableName};

/// Adds a variable specification into the supplied program.
/// Returns its index in the program.
pub fn variable(arena: &mut Program, spec: Value) -> Result<NodeId, &'static str> {
    let name = spec["name"].as_str()
                           .map(|name| {
                               VariableName { canonical: String::from(name) }
                           });
    let description = spec["description"].as_str().unwrap_or("");
    let vartype = spec["type"].as_str().ok_or("Variable requires \"type\" field.")?;

    let var = Variable {
        name: name,
        vartype: ScalarType::from(vartype),
    };

    Ok(arena.new_node(Box::new(var)))
}

pub fn variables(yaml: &HashMap<String, Value>) -> Program {
    let variables = yaml["variables"].as_sequence().unwrap();
    let mut arena = Arena::new();
    for i in variables.into_iter() {
        self::variable(&mut arena, i.clone());
    }
    arena
}
