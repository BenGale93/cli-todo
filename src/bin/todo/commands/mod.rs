pub mod add;
pub mod init;
pub mod list;

use rusqlite::Connection;
use todo::prelude::*;

pub use crate::commands::{add::AddArgs, init::InitArgs, list::ListArgs};

pub fn get_connection() -> Result<Connection> {
    let cfg: ToDoConfig = confy::load(APP_NAME, None)?;
    Ok(Connection::open(cfg.db_path())?)
}
