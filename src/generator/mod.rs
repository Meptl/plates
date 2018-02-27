/// Module for adding representation nodes to correct parts of the Program
pub mod php;

pub type Program = Arena<Code>;

pub struct Code {
    pub preamble: String
    pub body: String
    pub postamble: String
}

pub struct Output {

}

pub fn generate<T>(mut outstream: T, arena: &Program) where T: ::std::io::Write {
    // todo: traverse tree and output.
}
