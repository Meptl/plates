/// This module describes the internal representation of data.
/// It is not necessary for these types to be easily represented as a string.
pub struct DataTable<'a> {
    data: Vec<&'a Data>,
    type_indexer: HashMap<DataType, DataTableRefs>
}

/// Indices into the DataTable
type DataTableRefs = Vec<usize>;


pub enum DataType {
    Function,
    Scalar(ScalarType),
    Structure,
}

pub enum ScalarType
    Boolean,
    Char,
    Float,
    Integer,
    String, // TODO: Decide if this is its own datatype.
}

pub trait Data: ::mopa::Any {
    fn type(&self) -> DataType;
}

pub struct Structure {
    fields: DataTableRefs,
}

impl Data for Structure {
    fn type(&self) -> DataType {
        DataType::Structure
    }
}

pub struct Scalar {
    vartype: ScalarType
}

impl Data for Scalar {
    fn type(&self) -> DataType {
        DataType::Scalar(self.vartype)
    }
}

pub struct Function {
    statements: Vec<Statement>
}

impl Data for Function {
    fn type(&self) -> DataType {
        DataType::Function
    }
}

// todo: Statement
pub struct Statement {}
