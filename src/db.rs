use rusqlite::{Connection, Result};
use std::path::Path;

pub fn init_db() -> Result<Connection> {
    let db_path = "knowledge.db";
    let exists = Path::new(db_path).exists();

    let conn = Connection::open(db_path)?;

    if !exists {
        let schema = include_str!("schema/001_initial.sql");
        conn.execute_batch(schema)?;

        let seed = include_str!("seed/seed.sql");
        conn.execute_batch(seed)?;
    }

    Ok(conn)
}
