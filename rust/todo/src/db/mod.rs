use rusqlite::Connection;
use std::path::Path;

use crate::error::Result;

pub mod list;
pub mod list_item;
pub mod models;
pub mod schema;
pub mod store;

mod embedded {
    use refinery::embed_migrations;
    embed_migrations!("./migrations");
}

pub struct TodoDb {
    pub conn: Connection,
}

impl TodoDb {
    pub fn new(db_path: impl AsRef<Path>) -> Result<Self> {
        let mut conn = Connection::open(&db_path)?;
        embedded::migrations::runner().run(&mut conn)?;

        Ok(TodoDb { conn })
    }
}
