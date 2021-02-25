use rusqlite::{Connection, ToSql};

use crate::db::models::list_item::TodoListItem;
use crate::db::schema::TODO_LIST_ITEM_COMMON_COLS;
use crate::error::Result;

pub(crate) fn create(conn: &Connection, name: String, list_id: i64) -> Result<()> {
    let insert_sql = "INSERT INTO todo_list_items (name, todo_list_id) VALUES (?1, ?2)";

    conn.execute(&insert_sql, &[&name as &dyn ToSql, &list_id])?;

    Ok(())

    // let select_sql = "SELECT {common_cols} FROM todo_list_items WHERE id = ?1";
    // let mut stmt = conn.prepare(&select_sql)?;
    // let todo_list_item = stmt.query_row(
    //     &[&conn.last_insert_rowid() as &dyn ToSql],
    //     TodoListItem::from_row,
    // )?;

    // Ok(todo_list_item)
}

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

pub(crate) fn set_complete(conn: &Connection, id: i64, complete: bool) -> Result<()> {
    let sql = "UPDATE todo_list_items SET complete = ?1 WHERE id = ?2";

    conn.execute(&sql, &[&complete as &dyn ToSql, &id])?;

    Ok(())
}
