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
        remove_todo(self.name, &conn)?;
        Ok(())
    }
}
