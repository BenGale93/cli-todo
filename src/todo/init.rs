use std::path::Path;

use rusqlite::Connection;

use crate::prelude::*;

pub fn initialize_todo_db<P: AsRef<Path>>(path: P) -> Result<()> {
    let todo_db = path.as_ref();
    std::fs::create_dir_all(todo_db.parent().unwrap())?;
    let conn = Connection::open(todo_db)?;
    let _ = conn.execute(
        "CREATE TABLE IF NOT EXISTS todo (
            id INTEGER PRIMARY KEY,
            content TEXT NOT NULL,
            due DATETIME NOT NULL,
            status TEXT NOT NULL
        )",
        (),
    );
    Ok(())
}
