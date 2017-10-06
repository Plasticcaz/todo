use todo_item::TodoItem;

#[derive(Debug, Clone)]
pub enum TodoAppEvent {
    AddTodoAt(TodoItem, usize),
    RemoveTodoAt(TodoItem, usize),
    ToggleComplete(usize),
}

