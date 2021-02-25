CREATE TABLE todo_list_items (
       id INTEGER NOT NULL PRIMARY KEY,
       name TEXT NOT NULL,
       complete BOOLEAN NOT NULL DEFAULT false,
       todo_list_id INTEGER NOT NULL,

       FOREIGN KEY(todo_list_id) REFERENCES todo_lists(id)
);
