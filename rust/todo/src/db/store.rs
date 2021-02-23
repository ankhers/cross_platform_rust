use std::path::Path;
use std::sync::{Arc, Mutex};

use crate::db::TodoDb;
use crate::error::Result;

pub struct Store {
    db: Arc<Mutex<TodoDb>>,
}

impl Store {
    pub fn new(db_path: impl AsRef<Path>) -> Result<Self> {
        Ok(Self {
            db: Arc::new(Mutex::new(TodoDb::new(db_path)?)),
        })
    }
}
