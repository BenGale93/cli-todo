use rusqlite::Connection;

use crate::{prelude::*, sql_utils::execute_query};

pub fn remove_todo(name: &str, conn: &Connection) -> Result<usize> {
    execute_query(conn, "DELETE FROM todo WHERE name = (?1)", [&name])
}

pub fn remove_done(conn: &Connection) -> Result<usize> {
    execute_query(conn, "DELETE FROM todo WHERE status = 'Done'", [])
}
