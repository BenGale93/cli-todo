pub mod error;

pub mod prelude {
    pub use crate::error::ToDoError;

    pub type Result<T> = core::result::Result<T, ToDoError>;
}
