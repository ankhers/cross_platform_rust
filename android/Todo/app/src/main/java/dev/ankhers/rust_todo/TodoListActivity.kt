package dev.ankhers.rust_todo

import android.os.Bundle
import android.text.InputType
import android.widget.*
import androidx.appcompat.app.AlertDialog
import androidx.appcompat.app.AppCompatActivity
import dagger.hilt.android.AndroidEntryPoint
import dev.ankhers.todo.Todo
import dev.ankhers.todo.TodoListItem
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

        listView.setOnItemClickListener { parent, view, position, id ->
            val item = adapter.getItem(position)

            todo.listItemSetComplete(item.id, !item.complete)
            adapter.setItems(todo.getListItems(id))
            adapter.notifyDataSetChanged()
        }

        val builder = AlertDialog.Builder(this)
        builder.setTitle("New List Item")

        val input = EditText(this)

        input.setInputType(InputType.TYPE_CLASS_TEXT)
        builder.setView(input)

        builder.setPositiveButton(
            "Create"
        ) { dialog, which ->
            todo.createListItem(input.text.toString(), id)
            adapter.setItems(todo.getListItems(id))
            adapter.notifyDataSetChanged()
        }

        builder.setNegativeButton(
            "Cancel"
        ) { dialog, which -> dialog.cancel() }

        val add = findViewById<Button>(R.id.add_list_item)
        add.setOnClickListener {
            builder.show()
        }
    }
}