use chrono::{DateTime, Utc};
use clap::Args;
use todo::prelude::*;

use crate::commands::get_connection;

#[derive(Args)]
pub struct ListArgs {
    #[arg(short, long)]
    status: Option<Vec<Status>>,
    #[arg(short, long)]
    due: Option<DateTime<Utc>>,
}

impl ListArgs {
    pub fn run(self) -> Result<()> {
        let conn = get_connection()?;
        let todos = list_todos(self.status, self.due, &conn)?;
        let table = tabled::Table::new(todos);
        println!("{}", table);
        Ok(())
    }
}
