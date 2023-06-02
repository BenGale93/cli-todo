use chrono::{DateTime, Utc};
use clap::Args;
use todo::prelude::*;

use crate::commands::get_connection;

#[derive(Args)]
pub struct AddArgs {
    name: String,
    content: String,
    #[arg(value_parser = parse_datetime)]
    due: DateTime<Utc>,
}

impl AddArgs {
    pub fn run(self) -> Result<()> {
        let conn = get_connection()?;
        let todo = ToDo::new(self.name, self.content, self.due, Status::Due);
        add_todo(&todo, &conn)?;
        Ok(())
    }
}

fn parse_datetime(arg: &str) -> std::result::Result<DateTime<Utc>, chrono::ParseError> {
    Ok(DateTime::parse_from_str(arg, "%Y-%m-%d %H:%M:%S %z")?.into())
}
