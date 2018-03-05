use super::Data;

/// A collection of variables or functions.
/// A named context is analogous to a C struct.
struct Context {
    name: Option<String>
    fields: Vec<Arc<Data>>
}

impl Context {
    fn new_struct(name: &str, fields: Vec<Arc<Data>>) -> Context {
        Context {
            name: Some(name),
            fields: fields,
        }
    }

    fn new_context(fields: Vec<Arc<Data>>) -> Context {
        Context {
            name: None,
            fields: fields,
        }
    }

    fn name(&self) -> Option<String> {
        self.name
    }

    fn fields(&self) -> Vec<Arc<Data>> {
        self.fields
    }
}

impl Data for Context {
    fn data_type(&self) -> DataType {
        DataType::Context
    }
}
