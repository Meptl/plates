/// Module for adding representation nodes to correct parts of the Program
pub mod php;

pub type Program = Arena<Code>;

/// Code representation of singular expression. Note that this does not include any style outputs.
#[derive(Default)]
pub struct Code {
    pub preamble: Option<String>
    pub postamble: Option<String>
}

impl Code {
    pub fn new(pre: &str) -> Code {
        Code {
            preamble: Some(String::from(pre)),
            ..Default::default()
        }
    }

    pub fn new_code(pre: &str, post: &str) -> Code {
        Code {
            preamble: Some(String::from(pre),
            postamble: Some(String::from(post),
        }
    }

    /// Generate an empty node
    pub fn new_empty() -> Code {
        Default::default()
    }
}

/// Write a node's code output.
pub fn generate_node<T>(outstream: &mut T, arena: &Program, id: NodeId, depth) where T: ::std::io::Write {
    let code = arena[id];
    let indent = format!("{:indent$}", "", indent=depth);
    if let Some(pre) = code.preamble {
        outstream.write(code.preamble);
    }
    for child in id.children(arena) {
        generate_node(outstream, arena, child);
    }
    outstream.write(code.postamble);
}

/// Print the arena to outstream.
pub fn generate<T>(mut outstream: T, arena: &Program, root: NodeId) where T: ::std::io::Write {
    // Assume the first node is the root.
    generate_node(&mut outstream, arena, root);
    outstream.flush();
}
