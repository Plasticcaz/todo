use todo_item::TodoItem;

#[derive(Debug, Clone)]
pub enum TodoAppEvent {
    AddTodoAt(TodoItem, usize),
    RemoveTodoAt(TodoItem, usize),
    /// Replace the first item with the second item, at the index specified.
    ReplaceTodoAt(TodoItem, TodoItem, usize),
    ToggleComplete(usize),
}

