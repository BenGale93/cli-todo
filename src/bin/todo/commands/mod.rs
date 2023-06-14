pub mod add;
pub mod edit;
pub mod go;
pub mod init;
pub mod list;
pub mod rm;

use chrono::NaiveDateTime;
use rusqlite::Connection;
use todo::prelude::*;

pub use crate::commands::{
    add::AddArgs, edit::EditArgs, go::GoArgs, init::InitArgs, list::ListArgs, rm::RemoveArgs,
};

pub fn get_connection() -> Result<Connection> {
    let cfg: ToDoConfig = confy::load(APP_NAME, None)?;
    log::info!("Getting connection to: {}", cfg.db_path().display());
    Ok(Connection::open(cfg.db_path())?)
}

fn parse_datetime(arg: &str) -> std::result::Result<NaiveDateTime, chrono::ParseError> {
    arg.parse()
}
