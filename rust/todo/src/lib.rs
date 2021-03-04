pub mod db;
pub mod error;

#[cfg(target_os = "android")]
use android_logger::Config;
#[cfg(target_os = "android")]
use log::Level;

#[cfg(not(target_os = "android"))]
extern crate simple_logger;

use crate::db::models::list::TodoList;
use crate::db::models::list_item::TodoListItem;
use crate::db::store::Store;
use crate::error::Error;

pub fn setup() {
    #[cfg(target_os = "android")]
    android_logger::init_once(Config::default().with_min_level(Level::Trace));
    #[cfg(not(target_os = "android"))]
    simple_logger::SimpleLogger::new().init().unwrap();
}

include!(concat!(env!("OUT_DIR"), "/todo.uniffi.rs"));
