use clap::Args;
use colored::Colorize;
use todo::prelude::*;

use crate::commands::get_connection;

#[derive(Args)]
pub struct GoArgs {
    /// Name of the todo to advance to the next status. ToDo => In-Progress => Done.
    name: String,
}

impl GoArgs {
    pub fn run(self) -> Result<()> {
        let conn = get_connection()?;
        match go_todo(&self.name, &conn) {
            Ok(status) => println!(
                "Updated todo: '{}' to status: '{}'",
                &self.name.bold(),
                status.to_string().yellow()
            ),
            Err(e) => return Err(e),
        };
        Ok(())
    }
}
