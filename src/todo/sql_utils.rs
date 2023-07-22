use rusqlite::{Connection, Params};

use crate::prelude::*;

pub fn execute_query<P: Params>(conn: &Connection, query: &str, params: P) -> Result<usize> {
    let mut stmt = conn.prepare(query)?;

    let result = stmt.execute(params);

    log::info!("{:?}", stmt.expanded_sql());

    Ok(result?)
}
