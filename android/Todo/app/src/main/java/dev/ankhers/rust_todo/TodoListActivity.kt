package dev.ankhers.rust_todo

import android.os.Bundle
import android.widget.Button
import android.widget.ListView
import android.widget.TextView
import android.widget.Toast
import androidx.appcompat.app.AppCompatActivity
import dagger.hilt.android.AndroidEntryPoint
import dev.ankhers.todo.Todo
import javax.inject.Inject

@AndroidEntryPoint
class TodoListActivity : AppCompatActivity() {
    @Inject lateinit var todo: Todo
    lateinit var adapter: TodoListItemsAdapter

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_todo_list)

        val id = intent.extras!!.getLong("id")
        val todoList = todo.getList(id)

        val name = findViewById<TextView>(R.id.todo_list_name)
        name.text = todoList.name

        adapter = TodoListItemsAdapter(this, todo.getListItems(id))

        val listView = findViewById<ListView>(R.id.todo_list_items_list)
        listView.adapter = adapter

        val add = findViewById<Button>(R.id.add_list_item)
        add.setOnClickListener {
            Toast.makeText(applicationContext, "Need to add list item", Toast.LENGTH_LONG).show()
        }
    }
}