use core::result::Result as Res;

use rusqlite::{Connection, Error};

use crate::prelude::*;

pub fn go_todo(name: String, conn: &Connection) -> Result<()> {
    let mut stmt = conn.prepare("SELECT status FROM todo WHERE name = (?1)")?;

    let statuses = stmt.query_map([&name], |row| {
        Ok(row.get::<usize, String>(0)?.parse().unwrap())
    })?;

    let statuses = statuses.collect::<Res<Vec<Status>, Error>>()?;
    let status = statuses
        .first()
        .ok_or(ToDoError::Generic("To Do not found".to_string()))?;

    let new_status = match status {
        Status::ToDo => Status::InProgress,
        Status::InProgress => Status::Done,
        Status::Done => Status::Done,
    };

    let _ = conn.execute(
        "UPDATE todo SET status = (?1) WHERE name = (?2)",
        (new_status.to_string(), &name),
    )?;
    println!("Updated todo: '{name}' to status: '{new_status}'");
    Ok(())
}
