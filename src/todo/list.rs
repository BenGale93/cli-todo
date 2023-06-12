use core::result::Result as Res;

use chrono::{DateTime, Utc};
use rusqlite::{Connection, Error};

use crate::prelude::*;

pub fn list_todos(
    status: Option<Vec<Status>>,
    datetime: Option<DateTime<Utc>>,
    conn: &Connection,
) -> Result<Vec<ToDo>> {
    let datetime_where = datetime.map(|d| format!("due<='{}'", d.to_string()));

    let status_where = match status {
        Some(s) => {
            let v = s
                .into_iter()
                .map(|e| format!("'{}'", e))
                .collect::<Vec<String>>()
                .join(", ");
            Some(format!("status in ({})", v))
        }
        None => None,
    };

    let where_clause = [datetime_where, status_where]
        .into_iter()
        .flatten()
        .collect::<Vec<String>>()
        .join(" AND ");

    let query = match where_clause.as_str() {
        "" => "SELECT * FROM todo".to_string(),
        _ => format!("SELECT * FROM todo WHERE {where_clause}"),
    };

    let mut stmt = conn.prepare(&query)?;
    let todo_iter = stmt.query_map([], |row| {
        Ok(ToDo::new(
            row.get(0)?,
            row.get(1)?,
            row.get::<usize, DateTime<Utc>>(2)?,
            row.get::<usize, String>(3)?.parse().unwrap(),
        ))
    })?;
    let result = todo_iter.collect::<Res<Vec<ToDo>, Error>>()?;
    log::info!("{:?}", stmt.expanded_sql());
    Ok(result)
}
