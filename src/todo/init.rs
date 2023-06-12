use std::path::Path;

use rusqlite::Connection;

use crate::prelude::*;

pub fn initialize_todo_db<P: AsRef<Path>>(path: P) -> Result<()> {
    let todo_db = path.as_ref();
    std::fs::create_dir_all(todo_db.parent().unwrap())?;
    let conn = Connection::open(todo_db)?;

    let mut stmt = conn.prepare(
        "CREATE TABLE IF NOT EXISTS todo (
            name TEXT PRIMARY KEY,
            content TEXT NOT NULL,
            due DATETIME NOT NULL,
            status TEXT NOT NULL
        )",
    )?;

    let result = stmt.execute(()).map(|_| ());

    log::info!("{:?}", stmt.expanded_sql());

    Ok(result?)
}
