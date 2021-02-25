package dev.ankhers.rust_todo

import android.os.Bundle
import android.widget.Button
import android.widget.EditText
import android.widget.Toast
import androidx.appcompat.app.AppCompatActivity
import dagger.hilt.android.AndroidEntryPoint
import dev.ankhers.todo.ErrorException
import dev.ankhers.todo.Todo
import javax.inject.Inject

@AndroidEntryPoint
class NewListActivity : AppCompatActivity() {
    @Inject lateinit var todo: Todo

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_new_list)

        val save = findViewById<Button>(R.id.save)
        val input = findViewById<EditText>(R.id.new_list_name)

        save.setOnClickListener {
            try {
                todo.createList(input.text.toString())

                Toast.makeText(applicationContext, "New List Created!", Toast.LENGTH_LONG).show()
                finish()
            } catch(e: ErrorException.SqlError) {
                Toast.makeText(applicationContext, e.message, Toast.LENGTH_LONG).show()
            }
        }
    }
}
