/// Module for representing data.
pub mod variable;
pub mod decoder;

//pub use self::tree::DataStructure;

mod output;
use ::std::ops::Deref;
use ::indextree::Arena;

//pub struct Program(Arena<Box<Data>>);
pub type Program = Arena<Box<prelude::Data>>;

pub mod prelude {
    pub use super::output::Represent;
    /// Common trait of all tree nodes.
    pub trait Data {
        fn as_represent(&self) -> Option<&Represent> {
            None
        }
    }

    /// Common trait of all metadata.
    pub trait MetaData {}
}
