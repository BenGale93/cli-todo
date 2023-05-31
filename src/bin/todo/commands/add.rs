use chrono::{DateTime, Utc};
use clap::Args;
use rusqlite::Connection;
use todo::prelude::*;

#[derive(Args)]
pub struct AddArgs {
    name: String,
    content: String,
    #[arg(value_parser = parse_datetime)]
    due: DateTime<Utc>,
}

impl AddArgs {
    pub fn run(self) -> Result<()> {
        let cfg: ToDoConfig = confy::load(APP_NAME, None)?;
        let conn = Connection::open(cfg.db_path())?;
        let todo = ToDo::new(self.name, self.content, self.due, Status::Due);
        add_todo(&todo, &conn)?;
        Ok(())
    }
}

fn parse_datetime(arg: &str) -> std::result::Result<DateTime<Utc>, chrono::ParseError> {
    Ok(DateTime::parse_from_str(arg, "%Y-%m-%d %H:%M:%S %z")?.into())
}
