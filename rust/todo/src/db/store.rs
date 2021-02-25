use std::path::Path;
use std::sync::{Arc, Mutex};

use crate::db::models::{list::TodoList, list_item::TodoListItem};
use crate::db::TodoDb;
use crate::db::{list, list_item};
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

    pub fn create_list(&self, list_name: String) -> Result<()> {
        list::create(&self.db.lock().unwrap().conn, list_name)
    }

    pub fn get_lists(&self) -> Result<Vec<TodoList>> {
        list::get_all(&self.db.lock().unwrap().conn)
    }
    pub fn get_list(&self, id: i64) -> Result<TodoList> {
        list::get(&self.db.lock().unwrap().conn, id)
    }

    pub fn get_list_items(&self, id: i64) -> Result<Vec<TodoListItem>> {
        list_item::get_all_for_list(&self.db.lock().unwrap().conn, id)
    }
}
