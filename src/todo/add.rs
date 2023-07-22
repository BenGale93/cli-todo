use rusqlite::{Connection, Error, ErrorCode};

use crate::prelude::*;

pub fn add_todo(todo: &ToDo, conn: &Connection) -> Result<()> {
    let mut stmt =
        conn.prepare("INSERT INTO todo (name, content, due, status) VALUES (?1, ?2, ?3, ?4)")?;

    let result = stmt.execute((
        todo.name(),
        todo.content(),
        todo.due(),
        todo.status().to_string(),
    ));

    log::info!("{:?}", stmt.expanded_sql());
    match result {
        Ok(_) => Ok(()),
        Err(Error::SqliteFailure(e, _)) if matches!(e.code, ErrorCode::ConstraintViolation) => {
            Err(ToDoError::UniqueName(todo.name().to_string()))
        }
        Err(e) => Err(e.into()),
    }
}
