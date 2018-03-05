/// Also indexes data so that they are easily retrievable.

pub(crate) enum DataType {
    Context,
    Function,
    Scalar(ScalarType),
}

pub(crate) trait Data: ::mopa::Any {
    fn data_type(&self) -> DataType;
}
