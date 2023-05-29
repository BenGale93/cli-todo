#[derive(thiserror::Error, Debug)]
pub enum ToDoError {
    #[error("Generic error: {0}")]
    Generic(String),

    #[error(transparent)]
    IO(#[from] std::io::Error),
}
