use std::path::Path;

pub mod db;
pub mod error;

use crate::db::models::list::TodoList;
use crate::db::models::list_item::TodoListItem;
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

    pub fn create_list(&self, list_name: String) -> Result<(), Error> {
        self.store.create_list(list_name)
    }

    pub fn get_all_lists(&self) -> Result<Vec<TodoList>, Error> {
        self.store.get_lists()
    }

    pub fn get_list(&self, id: i64) -> Result<TodoList, Error> {
        self.store.get_list(id)
    }

    pub fn create_list_item(&self, name: String, list_id: i64) -> Result<(), Error> {
        self.store.create_list_item(name, list_id)
    }

    pub fn get_list_items(&self, id: i64) -> Result<Vec<TodoListItem>, Error> {
        self.store.get_list_items(id)
    }

    pub fn list_item_set_complete(&self, id: i64, complete: bool) -> Result<(), Error> {
        self.store.list_item_set_complete(id, complete)
    }
}

include!(concat!(env!("OUT_DIR"), "/todo.uniffi.rs"));
