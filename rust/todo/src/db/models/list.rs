use rusqlite::Row;

#[derive(Debug)]
pub struct TodoList {
    pub id: i64,
    pub name: String,
}

impl TodoList {
    pub fn from_row(row: &Row<'_>) -> Result<TodoList, rusqlite::Error> {
        Ok(Self {
            id: row.get("id")?,
            name: row.get("name")?,
        })
    }
}
