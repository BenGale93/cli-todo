use chrono::NaiveDateTime;
use clap::Args;
use colored::Colorize;
use todo::prelude::*;

use crate::commands::{get_connection, parse_datetime};

#[derive(Args)]
pub struct EditArgs {
    /// Name of the todo to edit.
    name: String,
    #[command(flatten)]
    fields: Fields,
}

#[derive(Args)]
#[group(required = true)]
struct Fields {
    /// The new content of the todo.
    #[arg(short)]
    content: Option<String>,
    /// When the todo is now due.
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
    #[arg(short, value_parser = parse_datetime)]
    due: Option<NaiveDateTime>,
}

impl EditArgs {
    pub fn run(self) -> Result<()> {
        let conn = get_connection()?;
        let todo_patch = ToDoPatch {
            name: self.name,
            content: self.fields.content,
            due: self.fields.due,
        };
        match edit_todo(&todo_patch, &conn) {
            Ok(_) => println!(
                "ToDo: '{}' changed {}",
                todo_patch.name.bold(),
                "successfully".green()
            ),
            Err(e) => return Err(e),
        };
        Ok(())
    }
}
