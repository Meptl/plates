/// Producer for PHP code.
use ::indextree::NodeId;
use ::repr::Program;
use ::repr::variable::{Variable, VariableType};
use ::unindent::unindent;
// TODO: We want these code segments to be stored somewhere...

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
        let i = NodeId(i);
        let is_struct = {
            let ref node = arena[i];
            match node.downcast_ref::<Variable>() {
                Some(var) => var.vartype == VariableType::Struct,
                None => false
            }
        };
        if is_struct {
            structs.push(i);
        }
    }

    for id in structs {
        let s = arena[id].downcast_ref::<Variable>().unwrap();
        // Preamble
        let mut output = format!("class {} {{\n", s.name.camel_case());
        // Fields
        for child in id.children(arena).into_iter() {
            let var = arena[child].downcast_ref::<Variable>().unwrap();
            let declaration = format!("
            /**
             * {description}.
             *
             * @var {var_type}
             */
            private ${var_name};\n",
            description = var.description.as_ref().unwrap(),
            var_type = var.vartype,
            var_name = var.name.lower_camel_case()
            );
            output.push_str(&declaration);
        }
        outstream.write(unindent(&output).as_bytes());
    }

    // TODO: indentation
    outstream.write(b"\n\n}\n");

    outstream.flush();
}

pub fn construct(data: &DataTable) -> Program {
    let program = &mut Arena::new();
    let mut root = program.new_node(Code { ..Default::default() });
    root.append(program, Code::new("<?php>"));
    root.append(program, Code::new(r#"namespace \Blah"#));

    for s in data.structures(program) {
        root.append(program, Code::new("public class");
    }
    root.append(program, Code
}
