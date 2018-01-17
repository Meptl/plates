/// Module for decoding specification representation of data into plates structures
/// Expects decoded Rust structures of a file. i.e. Values produced from serde-yaml.
use super::prelude::*;
use ::indextree::NodeId;
use ::serde_yaml::Value;
use super::variable::{ScalarType, Variable, VariableName};

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
