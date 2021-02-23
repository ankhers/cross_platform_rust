use std::path::Path;

pub mod db;
pub mod error;

use crate::db::store::Store;
use crate::error::Error;

pub struct Todo {
    store: Store,
}

impl Todo {
    pub fn new(db_path: impl AsRef<Path>) -> Result<Self, Error> {
        Ok(Todo {
            store: Store::new(db_path)?,
        })
    }
}

include!(concat!(env!("OUT_DIR"), "/todo.uniffi.rs"));
