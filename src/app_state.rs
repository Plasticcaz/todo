use todo_item::TodoItem;
use app_event::TodoAppEvent;

pub struct AppState {
    /// The list of todo items we wish to complete.
    todos: Vec<TodoItem>,
    /// Keeps track of the changes made to the todo list.
    changes: Vec<TodoAppEvent>,
    running: bool,
}

impl AppState {
    pub fn new() -> AppState {
        let todos = Vec::new();
        let changes = Vec::new();
        let running = true;

        AppState {
            todos,
            changes,
            running,
        }
    }

    pub fn add_todo(&mut self, item: TodoItem) {
        let add_index = self.todos.len();
        self.todos.push(item.clone());
        self.changes.push(TodoAppEvent::AddTodoAt(item, add_index));
    }

    pub fn replace_todo(&mut self, index: usize, item: TodoItem) {
        self.todos.push(item.clone());
        let prev = self.todos.swap_remove(index);
        self.changes.push(TodoAppEvent::ReplaceTodoAt(prev, item, index));
    }

    pub fn remove_todo(&mut self, index: usize) {
        let item = self.todos.remove(index);
        self.changes.push(TodoAppEvent::RemoveTodoAt(item, index));
    }
    
    pub fn toggle_complete(&mut self, index: usize) {
        let item = &mut self.todos[index];
        item.complete = !item.complete;
        self.changes.push(TodoAppEvent::ToggleComplete(index))
    }

    pub fn get_todo_list(&self) -> &[TodoItem] {
        &self.todos
    }

    pub fn get_changes(&self) -> &[TodoAppEvent] {
        &self.changes
    }

    pub fn is_running(&self) -> bool {
        self.running
    }

    pub fn quit(&mut self) {
        self.running = false;
    }
}
