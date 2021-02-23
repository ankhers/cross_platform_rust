package dev.ankhers.rust_todo

import android.content.Context
import dagger.Module
import dagger.Provides
import dagger.hilt.InstallIn
import dagger.hilt.android.qualifiers.ApplicationContext
import dagger.hilt.components.SingletonComponent
import dev.ankhers.todo.Todo

@Module
@InstallIn(SingletonComponent::class)
object TodoModule {
    @Provides
    fun provideTodo(@ApplicationContext context: Context): Todo {
        return Todo(context.getDatabasePath("todo.sqlite").absolutePath)
    }
}