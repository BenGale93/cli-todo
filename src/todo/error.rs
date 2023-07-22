#[derive(thiserror::Error, Debug)]
pub enum ToDoError {
    #[error(transparent)]
    IO(#[from] std::io::Error),

    #[error(transparent)]
    Sql(#[from] rusqlite::Error),

    #[error(transparent)]
    Chrono(#[from] chrono::ParseError),

    #[error(transparent)]
    Config(#[from] confy::ConfyError),

    #[error("Your ToDo needs a unique name, `{0}` is already in use")]
    UniqueName(String),

    #[error("Could not parse `{0}` into a date")]
    DateCode(String),

    #[error("ToDo not found")]
    NotFound,

    #[error("Unrecognised status: `{0}`")]
    UnrecognisedStatus(String),

    #[error("Config already exists")]
    ConfigExists,

    #[error("Option missing")]
    OptionMissing,
}
