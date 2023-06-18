#[derive(thiserror::Error, Debug)]
pub enum ToDoError {
    #[error("Generic error: {0}")]
    Generic(String),

    #[error(transparent)]
    IO(#[from] std::io::Error),

    #[error(transparent)]
    Sql(#[from] rusqlite::Error),

    #[error(transparent)]
    Chrono(#[from] chrono::ParseError),

    #[error(transparent)]
    Config(#[from] confy::ConfyError),
}
