use chrono::NaiveDateTime;
use clap::Args;
use colored::Colorize;
use todo::prelude::*;

use crate::commands::{get_connection, parse_datetime};

#[derive(Args)]
pub struct AddArgs {
    /// A short name for the todo.
    name: String,
    /// What you actually need to do.
    content: String,
    /// When the todo is due.
    ///
    /// Acceptable formats include:
    ///
    /// %Y-%m-%d %H%M
    ///
    /// Short day code: e.g. Mon
    ///
    /// Short day code and time: e.g. Mon1700
    ///
    /// Count and frequency: e.g. 2d, 1w, 3m, 4y.
    ///
    /// Count, frequency and time: e.g. 2d1700.
    #[arg(value_parser = parse_datetime)]
    due: NaiveDateTime,
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
