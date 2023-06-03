use clap::Args;
use todo::prelude::*;

use crate::commands::get_connection;

#[derive(Args)]
pub struct GoArgs {
    name: String,
}

impl GoArgs {
    pub fn run(self) -> Result<()> {
        let conn = get_connection()?;
        go_todo(self.name, &conn)?;
        Ok(())
    }
}
