use rusqlite::{Connection, Error, ErrorCode};

use crate::prelude::*;

pub fn add_todo(todo: &ToDo, conn: &Connection) -> Result<()> {
    let result = conn.execute(
        "INSERT INTO todo (name, content, due, status) VALUES (?1, ?2, ?3, ?4)",
        (
            todo.name(),
            todo.content(),
            todo.due(),
            todo.status().to_string(),
        ),
    );
    match result {
        Ok(_) => {
            println!("ToDo: '{}' added successfully", todo.name());
            Ok(())
        }
        Err(Error::SqliteFailure(e, _)) if matches!(e.code, ErrorCode::ConstraintViolation) => Err(
            ToDoError::Generic("Your ToDo needs a unique name".to_string()),
        ),
        Err(e) => Err(e.into()),
    }
}
