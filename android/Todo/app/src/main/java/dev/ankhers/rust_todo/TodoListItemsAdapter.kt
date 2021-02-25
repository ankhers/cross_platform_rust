package dev.ankhers.rust_todo

import android.app.Activity
import android.content.Context
import android.view.LayoutInflater
import android.view.View
import android.view.ViewGroup
import android.widget.BaseAdapter
import android.widget.CheckedTextView
import dev.ankhers.todo.TodoListItem

class TodoListItemsAdapter(private val activity: Activity, private var items: List<TodoListItem>) : BaseAdapter() {
    private val inflater: LayoutInflater = activity.getSystemService(Context.LAYOUT_INFLATER_SERVICE) as LayoutInflater

    override fun getCount(): Int {
       return items.size
    }

    override fun getItem(position: Int): TodoListItem {
        return items[position]
    }

    override fun getItemId(position: Int): Long {
        return getItem(position).id
    }

    override fun getView(position: Int, convertView: View?, parent: ViewGroup?): View {
        val view = convertView ?: inflater.inflate(R.layout.todo_list_item_item, null)

        val todoListItem = getItem(position)
        val name = view.findViewById<CheckedTextView>(R.id.item_name)

        name.text = todoListItem.name
        name.isChecked = todoListItem.complete

        return view
    }

    fun setItems(newItems: List<TodoListItem>) {
        items  = newItems
    }
}