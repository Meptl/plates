/// Module containing tree representation of program structure.
use super::prelude::*;

pub struct DataStructure {
    parent: Option<Node>,
    children: Vec<Node>,
}

impl DataStructure {
    pub fn new() -> DataStructure {
        DataStructure {
            parent: None,
            children: Vec::new(),
        }
    }
}

impl Data for DataStructure {
    fn as_represent(&self) -> Option<&Represent> {
        None
    }
}
