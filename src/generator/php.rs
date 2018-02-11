/// Producer for PHP code.
use ::repr::Program;

pub fn generate<T>(mut outstream: T, yaml: &Program) where T: ::std::io::Write {
    outstream.write(b"Hi");
    outstream.flush();
}
