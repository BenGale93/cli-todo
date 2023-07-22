use clap::Args;
use todo::prelude::*;

use crate::commands::get_connection;

#[derive(Args)]
#[group(multiple = false)]
pub struct RemoveArgs {
    /// Name of the todo to delete.
    name: Option<String>,
    #[arg(short, long)]
    done: bool,
}

impl RemoveArgs {
    pub fn run(self) -> Result<()> {
        let conn = get_connection()?;

        match (self.name, self.done) {
            (Some(n), _) => {
                let result = remove_todo(&n, &conn)?;
                if result > 0 {
                    println!("ToDo: '{}' removed successfully", &n);
                } else {
                    println!("No ToDo named: '{}' was found", &n);
                }
            }
            (None, true) => {
                let result = remove_done(&conn)?;
                if result > 0 {
                    println!("All 'done' todos removed successfully");
                } else {
                    println!("No 'done' todos found");
                }
            }
            (None, false) => println!("No arguments provided"),
        };

        Ok(())
    }
}
