use rusqlite::{Connection, ToSql, NO_PARAMS};

use crate::db::models::list_item::TodoListItem;
use crate::db::schema::TODO_LIST_ITEM_COMMON_COLS;
use crate::error::Result;

pub(crate) fn get_all_for_list(conn: &Connection, id: i64) -> Result<Vec<TodoListItem>> {
    let sql = format!(
        "SELECT {common_cols} FROM todo_list_items WHERE todo_list_id = ?1 ORDER BY name ASC",
        common_cols = TODO_LIST_ITEM_COMMON_COLS
    );

    let mut stmt = conn.prepare(&sql)?;
    let list_items = stmt
        .query_map(&[&id as &dyn ToSql], TodoListItem::from_row)?
        .collect::<std::result::Result<Vec<TodoListItem>, _>>()?;

    Ok(list_items)
}
