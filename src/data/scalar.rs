#[derive(Copy, Clone)]
pub enum ScalarType
    Boolean,
    Char,
    Float,
    Integer,
    String, // This may eventually have to be its own DataType
}

/// A primitive type
pub struct Scalar {
    name: String,
    scalar_type: ScalarType
}

impl Scalar {
    fn new(name: &str, scalar_type: ScalarType) -> Scalar {
        Scalar {
            name: name,
            scalar_type: scalar_type,
        }
    }

    fn name(&self) -> String {
        self.name
    }

    fn scalar_type(&self) -> ScalarType {
        self.scalar_type
    }
}

impl Data for Scalar {
    fn data_type(&self) -> DataType {
        DataType::Scalar(self.scalar_type))
    }
}
