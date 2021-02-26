package dev.ankhers.rust_todo

import android.content.Context
import dagger.Module
import dagger.Provides
import dagger.hilt.InstallIn
import dagger.hilt.android.qualifiers.ApplicationContext
import dagger.hilt.components.SingletonComponent
import dev.ankhers.todo.Store

@Module
@InstallIn(SingletonComponent::class)
object TodoModule {
    @Provides
    fun provideStore(@ApplicationContext context: Context): Store {
        return Store(context.getDatabasePath("todo.sqlite").absolutePath)
    }
}