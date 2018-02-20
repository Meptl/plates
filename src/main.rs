#![feature(conservative_impl_trait)]
#![feature(try_from)]
#[macro_use]
extern crate mopa;
extern crate serde_yaml;
extern crate indextree;
extern crate itertools;
extern crate unindent;

use indextree::Arena;
use std::rc::Rc;
use std::fs::File;
use std::collections::HashMap;
use std::io::Write;
use serde_yaml::Value;
use repr::variable::{Variable, VariableName};

mod repr;
mod generator;

const USAGE: &'static str = "
Plates generates code from YAML.
    Usage: plates YAML_FILE [OUTPUT_FILE]
";

fn main() {
    let spec_name = std::env::args().skip(1).next().expect(USAGE);
    let spec_file = File::open(&spec_name).unwrap();
    let yaml: HashMap<String, Value> = serde_yaml::from_reader(&spec_file).unwrap();

    match repr::decoder::verify(&yaml) {
        Some(err) => println!("Invalid specification: {}", err),
        None => {},
    }

    let mut data = Arena::new();
    for variable in yaml["variables"].as_sequence().unwrap() {
        let idx = repr::decoder::variable(&mut data, variable).unwrap();
    }
    for structure in yaml["structs"].as_sequence().unwrap() {
        let idx = repr::decoder::structure(&mut data, structure).unwrap();
    }
    let native_nodes = data.len();

    // todo: Take a CLI flag for language specification
    generator::php::output(std::io::stdout(), &data);
}
