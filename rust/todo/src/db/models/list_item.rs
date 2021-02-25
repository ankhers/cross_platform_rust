use rusqlite::Row;

#[derive(Debug)]
pub struct TodoListItem {
    pub id: i64,
    pub name: String,
    pub complete: bool,
    pub todo_list_id: i64,
}

impl TodoListItem {
    pub fn from_row(row: &Row<'_>) -> Result<TodoListItem, rusqlite::Error> {
        Ok(Self {
            id: row.get("id")?,
            name: row.get("name")?,
            complete: row.get("complete")?,
            todo_list_id: row.get("todo_list_id")?,
        })
    }
}
