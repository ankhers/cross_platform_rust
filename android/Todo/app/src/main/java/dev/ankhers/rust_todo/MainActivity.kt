package dev.ankhers.rust_todo

import android.content.Intent
import androidx.appcompat.app.AppCompatActivity
import android.os.Bundle
import android.widget.ListView
import com.google.android.material.floatingactionbutton.FloatingActionButton
import dagger.hilt.android.AndroidEntryPoint
import dev.ankhers.todo.Store
import javax.inject.Inject

@AndroidEntryPoint
class MainActivity : AppCompatActivity() {
    @Inject lateinit var store: Store
    lateinit var adapter: TodoListAdapter

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_main)

        val addList = findViewById<FloatingActionButton>(R.id.add_list_item)

        addList.setOnClickListener {
            val intent = Intent(this, NewListActivity::class.java)
            this.startActivity(intent)
        }

        val listView = findViewById<ListView>(R.id.list_names)
        adapter = TodoListAdapter(this, store.getAllLists())
        listView.adapter = adapter

        listView.setOnItemClickListener { parent, view, position, id ->
            val intent = Intent(this, TodoListActivity::class.java)
            intent.putExtra("id", id)
            this.startActivity(intent)
        }
    }

    override fun onResume() {
        super.onResume()

        // TODO: If I return a TodoList from the creation, is there a way for me to just insert the record here instead of calling to get all the records again?
        adapter.setItems(store.getAllLists())
        adapter.notifyDataSetChanged()
    }
}