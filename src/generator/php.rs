/// Producer for PHP code.
use ::indextree::NodeId;
use ::repr::Program;
use ::repr::variable::{Variable, VariableType};
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

pub fn output<T>(mut outstream: T, arena: &Program) where T: ::std::io::Write {
    let preamble = preamble().into_bytes();
    outstream.write(&preamble);

    // todo: My indexree will probably store a HashMap for these. For now another linear search.
    let mut structs = Vec::new();
    for i in 0..arena.len() {
        let i = NodeId::from(i);
        let is_struct = {
            let ref node = arena[i];
            match node.data.downcast_ref::<Variable>() {
                Some(var) => var.vartype == VariableType::Struct,
                None => false
            }
        };
        if is_struct {
            structs.push(i);
        }
    }

    for id in structs {
        let s = arena[id].data.downcast_ref::<Variable>().unwrap();
        let class_name = s.name.camel_case();
        // Preamble
        let mut output = format!("class {} {{\n", class_name);
        // Fields
        for child in id.children(arena).into_iter() {
            let var = arena[child].data.downcast_ref::<Variable>().unwrap();
            let declaration = format!("
            /**
             * {description}.
             */
            private ${var_name};\n",
            description = var.description.as_ref().unwrap(),
            var_name = var.name.lower_camel_case()
            );
            output.push_str(&declaration);
        }
        outstream.write(unindent(&output).as_bytes());
    }

    outstream.flush();
}
