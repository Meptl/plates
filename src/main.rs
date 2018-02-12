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

    let variables = repr::decoder::variables(&yaml);
    let native_nodes = variables.count();

    // todo: Take a CLI flag for language specification
    generator::php::output(std::io::stdout(), &variables);
}
