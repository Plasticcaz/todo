
fn main() {
    let mut state = AppState::new();
    state.add_todo(TodoItem::new("Actually do some programming.", false));
    state.add_todo(TodoItem::new("Actually do other stuff.", false));
    state.remove_todo(0);
    state.replace_todo(0, TodoItem::new("Actually, do this instead.", false));

    println!("{:?}", state.get_todo_list());
    println!("{:?}", state.get_changes());
}

struct AppState {
    /// The list of todo items we wish to complete.
    todos: Vec<TodoItem>,
    /// Keeps track of the changes made to the todo list.
    changes: Vec<TodoAppEvent>
}

impl AppState {
    pub fn new() -> AppState {
        let todos = Vec::new();
        let changes = Vec::new();

        AppState {
            todos,
            changes,
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

    pub fn get_todo_list(&self) -> &[TodoItem] {
        &self.todos
    }

    pub fn get_changes(&self) -> &[TodoAppEvent] {
        &self.changes
    }
}

#[derive(Debug, Clone)]
enum TodoAppEvent {
    AddTodoAt(TodoItem, usize),
    RemoveTodoAt(TodoItem, usize),
    /// Replace the first item with the second item, at the index specified.
    ReplaceTodoAt(TodoItem, TodoItem, usize),
}

#[derive(Debug, Clone)]
struct TodoItem {
    /// The description of the item we wish to complete.
    pub description: String,
    /// Whether or not this item is complete.
    pub complete: bool,
}

impl TodoItem {
    pub fn new<S: Into<String>>(description: S, complete: bool) -> TodoItem {
        let description = description.into();
        TodoItem {
            description,
            complete
        }
    }
}