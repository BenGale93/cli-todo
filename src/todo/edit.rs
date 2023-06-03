use rusqlite::{Connection, Error, ErrorCode};

use crate::prelude::*;

pub fn edit_todo(todo_patch: &ToDoPatch, conn: &Connection) -> Result<()> {
    let content_clause = todo_patch
        .content
        .as_ref()
        .map(|c| format!("content = '{c}'"));

    let due_clause = todo_patch
        .due
        .map(|d| format!("due = '{}'", d.format("%F %T%.f%:z").to_string()));

    let set_clause = [content_clause, due_clause]
        .into_iter()
        .flatten()
        .collect::<Vec<String>>()
        .join(", ");

    let query = match set_clause.as_str() {
        "" => return Err(ToDoError::Generic("Nothing to update".to_string())),
        _ => format!(
            "UPDATE todo SET {set_clause} WHERE name = '{}'",
            todo_patch.name
        ),
    };

    let result = conn.execute(&query, ());
    match result {
        Ok(_) => {
            println!("ToDo: '{}' changed successfully", todo_patch.name);
            Ok(())
        }
        Err(Error::SqliteFailure(e, ec)) => match e.code {
            ErrorCode::ConstraintViolation => Err(ToDoError::Generic(
                "Your ToDo needs a unique name.".to_string(),
            )),
            _ => Err(Error::SqliteFailure(e, ec).into()),
        },
        Err(e) => Err(e.into()),
    }
}
