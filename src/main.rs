#![feature(conservative_impl_trait)]
#![feature(try_from)]
extern crate serde_yaml;
extern crate indextree;
extern crate itertools;

use indextree::Arena;
use std::rc::Rc;
use std::fs::File;
use std::collections::HashMap;
use std::io::Write;
use serde_yaml::Value;
use repr::{DataStructure};
use repr::variable::{Variable, VariableName};

mod repr;

const USAGE: &'static str = "
Plates generates code from YAML.
    Usage: plates YAML_FILE
";

fn main() {
    let file_name = std::env::args().skip(1).next().expect(USAGE);
    let file = File::open(&file_name).unwrap();
    let yaml: HashMap<String, Value> = serde_yaml::from_reader(&file).unwrap();
    let mut arena = Arena::new();

    let variables = yaml["variables"].as_sequence().unwrap();
    for i in variables.into_iter() {
        repr::decoder::variable(&mut arena, i.clone());
    }

    //let output = class.as_represent().unwrap().represent(String::new());
    //println!("{}", output);
    //
    let v = VariableName { canonical: String::from("This_is_AnArbitrary string-sentence") };

    println!("{}", v.snake_case());
}
