use chrono::NaiveDateTime;
use clap::Args;
use todo::prelude::{Status, *};

use crate::commands::get_connection;

#[derive(Args)]
#[group(multiple = false)]
struct StatusFilter {
    /// Whether to display all items. By default, done items are hidden.
    #[arg(short, long)]
    all: bool,
    /// Status to filter the full list on.
    #[arg(short, long)]
    status: Option<Vec<Status>>,
}

#[derive(Args)]
pub struct ListArgs {
    #[command(flatten)]
    status_filter: StatusFilter,
    /// Due date to filter on. Displays those due before the date given.
    #[arg(short, long)]
    due: Option<NaiveDateTime>,
}

impl ListArgs {
    pub fn run(self) -> Result<()> {
        let conn = get_connection()?;

        let statuses = match (self.status_filter.all, self.status_filter.status) {
            (false, None) => Some(vec![Status::ToDo, Status::InProgress]),
            (true, _) => None,
            (_, Some(s)) => Some(s),
        };

        let todos = list_todos(statuses, self.due, &conn)?;
        let todo_rows: Vec<ToDoRow> = todos.into_iter().map(|t| t.into()).collect();
        let table = tabled::Table::new(todo_rows);
        println!("{}", table);
        Ok(())
    }
}
