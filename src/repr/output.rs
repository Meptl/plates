use super::prelude::*;

/// Nodes that will produce output.
pub trait Represent {
    /// Return string intended to introduce this piece of data.
    fn preamble(&self) -> &str;

    /// Return string intended to represent this piece of data.
    fn body(&self) -> &str;

    /// Return string intended to close this piece of data.
    fn closing(&self) -> &str;
}

/// Metadata that has an output representation
pub struct OutputMeta {
    preamble: &'static str,
    body: &'static str,
    closing: &'static str,
}

impl MetaData for OutputMeta {}
impl Represent for OutputMeta {
    fn preamble(&self) -> &str {
        self.preamble
    }

    fn body(&self) -> &str {
        self.body
    }

    fn closing(&self) -> &str {
        self.closing
    }
}

// Build the output String for the data Node
/*
fn represent<D>(node: D, mut output: String) -> String where D: Data {
    output.push_str(self.preamble());
    for field in &self.fields {
        if let Some(repr) = field.as_represent() {
           output = repr.represent(output);
        }
    }
    output.push_str(self.closing());

    output
}
*/
