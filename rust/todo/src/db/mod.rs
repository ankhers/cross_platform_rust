use rusqlite::Connection;
use std::path::Path;

use crate::error::Result;

pub mod store;

pub struct TodoDb {
    pub conn: Connection,
}

impl TodoDb {
    pub fn new(db_path: impl AsRef<Path>) -> Result<Self> {
        Ok(TodoDb {
            conn: Connection::open(&db_path)?,
        })
    }
}
