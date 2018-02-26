/// Module for decoding specification representation of data into plates structures
/// Expects decoded Rust structures of a file. i.e. Values produced from serde-yaml.
use ::indextree::NodeId;
use ::serde_yaml::Value;
use ::std::collections::HashMap;
use ::std::ops::Deref;
use ::std::str::FromStr;
use super::super::prelude::*;
use super::super::Program;
use super::super::variable::{VariableType, Variable, VariableName};

/// Adds a variable specification into the supplied program.
/// Returns its index in the program.
pub fn variable(arena: &mut Program, spec: &Value) -> Result<NodeId, &'static str> {
    let name = spec["name"].as_str().unwrap();
    let description = spec["description"].as_str().unwrap_or("");
    let vartype = spec["type"].as_str().ok_or("Variable requires \"type\" field.")?;

    let var = Variable {
        name: VariableName { canonical: String::from(name) },
        vartype: VariableType::from_str(vartype).unwrap(),
        description: Some(String::from(description))
    };

    Ok(arena.new_node(Box::new(var)))
}

pub fn structure(arena: &mut Program, spec: &Value) -> Result<NodeId, &'static str> {
    let name = spec["name"].as_str().unwrap();
    let description = spec["description"].as_str().unwrap_or("");
    let structure = Variable {
        name: VariableName { canonical: String::from(name) },
        vartype: VariableType::Struct,
        description: Some(String::from(description))
    };
    let mut new_node = arena.new_node(Box::new(structure));

    for field in spec["fields"].as_sequence().unwrap() {
        let var_name = field.as_str().unwrap();
        // todo: linear search may not be ideal.
        // Find the variable in the arena.
        for i in 0..arena.len() {
            let i = NodeId(i);
            let is_child = {
                let ref data = arena[i];
                match data.downcast_ref::<Variable>() {
                    Some(var) => var.name.canonical == var_name,
                    None => false
                }
            };
            if is_child {
                new_node.append(i, arena);
            }
        }
    }

    Ok(new_node)
}
