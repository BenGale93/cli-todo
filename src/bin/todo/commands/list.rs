use chrono::NaiveDateTime;
use clap::Args;
use todo::prelude::*;

use crate::commands::get_connection;

#[derive(Args)]
pub struct ListArgs {
    #[arg(short, long)]
    status: Option<Vec<Status>>,
    #[arg(short, long)]
    due: Option<NaiveDateTime>,
}

impl ListArgs {
    pub fn run(self) -> Result<()> {
        let conn = get_connection()?;
        let todos = list_todos(self.status, self.due, &conn)?;
        let todo_rows: Vec<ToDoRow> = todos.into_iter().map(|t| t.into()).collect();
        let table = tabled::Table::new(todo_rows);
        println!("{}", table);
        Ok(())
    }
}
