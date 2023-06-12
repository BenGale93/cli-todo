use chrono::{DateTime, Utc};
use clap::Args;
use colored::Colorize;
use todo::prelude::*;

use crate::commands::{get_connection, parse_datetime};

#[derive(Args)]
pub struct EditArgs {
    name: String,
    #[command(flatten)]
    fields: Fields,
}

#[derive(Args)]
#[group(required = true)]
struct Fields {
    #[arg(short)]
    content: Option<String>,
    #[arg(short, value_parser = parse_datetime)]
    due: Option<DateTime<Utc>>,
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
