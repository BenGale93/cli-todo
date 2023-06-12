use clap::Args;
use todo::prelude::*;

use crate::commands::get_connection;

#[derive(Args)]
pub struct RemoveArgs {
    name: String,
}

impl RemoveArgs {
    pub fn run(self) -> Result<()> {
        let conn = get_connection()?;
        let result = remove_todo(&self.name, &conn)?;
        if result > 0 {
            println!("ToDo: '{}' removed successfully", &self.name);
        } else {
            println!("No ToDo named: '{}' was found", &self.name);
        }
        Ok(())
    }
}
