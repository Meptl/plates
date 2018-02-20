/// Producer for PHP code.
use ::repr::Program;
use ::unindent::unindent;

/// Create a preamble for php code.
fn preamble() -> String {
    let preamble = unindent(&format!("
        <?php>

        namespace {};
        ", "Stub"));
    // todo: Figure out namespace and includes.
    preamble
}

pub fn output<T>(mut outstream: T, yaml: &Program) where T: ::std::io::Write {
    let preamble = preamble().into_bytes();
    outstream.write(&preamble);

    outstream.flush();
}
