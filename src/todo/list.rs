use core::result::Result as Res;

use chrono::NaiveDateTime;
use rusqlite::{Connection, Error};

use crate::prelude::*;

pub fn list_todos(
    status: Option<&[Status]>,
    datetime: Option<NaiveDateTime>,
    conn: &Connection,
) -> Result<Vec<ToDo>> {
    let datetime_where = datetime.map(|d| format!("due<='{}'", d));

    let status_where = status.map(|s| {
        let v = s
            .iter()
            .map(|e| format!("'{}'", e))
            .collect::<Vec<String>>()
            .join(", ");
        format!("status in ({})", v)
    });

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
            row.get::<usize, NaiveDateTime>(2)?,
            row.get::<usize, String>(3)?.parse().unwrap(),
        ))
    })?;
    let result = todo_iter.collect::<Res<Vec<ToDo>, Error>>()?;
    log::info!("{:?}", stmt.expanded_sql());
    Ok(result)
}
