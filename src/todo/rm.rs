use rusqlite::Connection;

use crate::prelude::*;

pub fn remove_todo(name: String, conn: &Connection) -> Result<()> {
    let result = conn.execute("DELETE FROM todo WHERE name = (?1)", [&name])?;
    if result > 0 {
        println!("ToDo: '{}' removed successfully", name);
    } else {
        println!("No ToDo named: '{}' was found", name);
    }
    Ok(())
}
