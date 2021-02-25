package dev.ankhers.rust_todo

import android.app.Activity
import android.content.Context
import android.view.LayoutInflater
import android.view.View
import android.view.ViewGroup
import android.widget.BaseAdapter
import android.widget.TextView
import dev.ankhers.todo.TodoList

class TodoListAdapter(private val activity: Activity, private var items: List<TodoList>) : BaseAdapter() {
    private val inflater: LayoutInflater = activity.getSystemService(Context.LAYOUT_INFLATER_SERVICE) as LayoutInflater

    override fun getCount(): Int {
        return items.size
    }

    override fun getItem(position: Int): TodoList {
        return items[position]
    }

    override fun getItemId(position: Int): Long {
        return items[position].id
    }

    override fun getView(position: Int, convertView: View?, parent: ViewGroup?): View {
        val view = convertView ?: inflater.inflate(R.layout.todo_list_item, null)

        val todoList = getItem(position)
        val name = view.findViewById<TextView>(R.id.todo_list_item_name)

        name.text = todoList.name

        return view
    }

    fun setItems(newItems: List<TodoList>) {
        items  = newItems
    }
}