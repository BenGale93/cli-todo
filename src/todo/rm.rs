use rusqlite::Connection;

use crate::prelude::*;

pub fn remove_todo(name: &str, conn: &Connection) -> Result<usize> {
    let mut stmt = conn.prepare("DELETE FROM todo WHERE name = (?1)")?;

    let result = stmt.execute([&name]);

    log::info!("{:?}", stmt.expanded_sql());

    Ok(result?)
}
