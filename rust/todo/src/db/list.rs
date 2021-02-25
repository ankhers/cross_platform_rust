use rusqlite::{Connection, ToSql, NO_PARAMS};

use crate::db::models::list::TodoList;
use crate::db::schema::TODO_LIST_COMMON_COLS;
use crate::error::Result;

pub(crate) fn create(conn: &Connection, name: String) -> Result<()> {
    let sql = "INSERT INTO todo_lists (name) VALUES (?1)";

    conn.execute(&sql, &[&name as &dyn ToSql])?;

    Ok(())
}

pub(crate) fn get_all(conn: &Connection) -> Result<Vec<TodoList>> {
    let sql = format!(
        "SELECT {common_cols} FROM todo_lists ORDER BY name ASC",
        common_cols = TODO_LIST_COMMON_COLS
    );

    let mut stmt = conn.prepare(&sql)?;
    let lists = stmt
        .query_map(NO_PARAMS, TodoList::from_row)?
        .collect::<std::result::Result<Vec<TodoList>, _>>()?;

    Ok(lists)
}

pub(crate) fn get(conn: &Connection, id: i64) -> Result<TodoList> {
    let sql = format!(
        "SELECT {common_cols} FROM todo_lists WHERE id = ?1",
        common_cols = TODO_LIST_COMMON_COLS
    );

    let mut stmt = conn.prepare(&sql)?;
    let todo_list = stmt.query_row(&[&id as &dyn ToSql], TodoList::from_row)?;

    Ok(todo_list)
}
