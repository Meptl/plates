/// Module for representing data.
pub(crate) mod output;
pub(crate) mod util;

pub mod variable;


mod tree;
pub use self::tree::DataStructure;

mod prelude {
    pub use super::output::Represent;
    use ::indextree::Arena;
    use ::std::ops::Deref;

    pub struct Program(Arena<Box<Data>>);
    pub type Node = usize;

    /// Common trait of all tree nodes.
    pub trait Data {
        fn as_represent(&self) -> Option<&Represent> {
            None
        }
    }

    /// Common trait of all metadata.
    pub trait MetaData {}
}

