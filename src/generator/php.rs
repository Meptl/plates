/// Producer for PHP code.
use ::repr::Program;

pub fn output<T>(mut outstream: T, yaml: &Program) where T: ::std::io::Write {
    outstream.write(b"Hi");
    outstream.flush();
}
