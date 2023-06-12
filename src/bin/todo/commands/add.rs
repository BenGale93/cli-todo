use chrono::{DateTime, Utc};
use clap::Args;
use colored::Colorize;
use todo::prelude::*;

use crate::commands::{get_connection, parse_datetime};

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
        let todo = ToDo::new(self.name, self.content, self.due, Status::ToDo);
        match add_todo(&todo, &conn) {
            Ok(_) => println!(
                "ToDo: '{}' added {}",
                todo.name().bold(),
                "successfully".green()
            ),
            Err(e) => return Err(e),
        };
        Ok(())
    }
}
