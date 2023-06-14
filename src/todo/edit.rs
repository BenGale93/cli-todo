use rusqlite::{Connection, Error, ErrorCode};

use crate::prelude::*;

pub fn edit_todo(todo_patch: &ToDoPatch, conn: &Connection) -> Result<()> {
    let content_clause = todo_patch
        .content
        .as_ref()
        .map(|_| "content = ?".to_string());

    let due_clause = todo_patch.due.map(|_| "due = ?".to_string());

    let set_clause = [content_clause, due_clause]
        .into_iter()
        .flatten()
        .collect::<Vec<String>>()
        .join(", ");

    let query = match set_clause.as_str() {
        "" => return Err(ToDoError::Generic("Nothing to update".to_string())),
        _ => format!("UPDATE todo SET {set_clause} WHERE name = ?"),
    };

    let mut params = vec![];

    if let Some(c) = &todo_patch.content {
        params.push(c);
    }

    let date: String;
    if let Some(d) = &todo_patch.due {
        date = d.format("%F %T%.f").to_string();
        params.push(&date);
    }

    params.push(&todo_patch.name);

    let mut stmt = conn.prepare(&query)?;
    let result = stmt.execute(rusqlite::params_from_iter(params));

    log::info!("{:?}", stmt.expanded_sql());
    match result {
        Ok(_) => Ok(()),
        Err(Error::SqliteFailure(e, _)) if matches!(e.code, ErrorCode::ConstraintViolation) => Err(
            ToDoError::Generic("Your ToDo needs a unique name".to_string()),
        ),
        Err(e) => Err(e.into()),
    }
}
