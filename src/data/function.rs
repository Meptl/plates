/// A collection of Expressions.
struct Function {
    name: Option<String>,
    fields: Vec<Expression>
}

impl Function {
    fn new_function(name: &str, fields: Vec<Arc<Data>>) -> Function {
        Function {
            name: Some(name),
            fields: fields,
        }
    }

    fn new_lambda(fields: Vec<Arc<Data>>) -> Function {
        Function {
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


// Todo: This.
enum Expression {
    Assignment(Expression, Expression),
    Addition(Left, Right)
}


impl Data for Function {
    fn data_type(&self) -> DataType {
        DataType::Function
    }
}
