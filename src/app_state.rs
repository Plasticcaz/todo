use app_event::TodoAppEvent;
use todo_item::TodoItem;

const FILEPATH: &str = "db.todo";

pub struct AppState {
    /// The list of todo items we wish to complete.
    todos: Vec<TodoItem>,
    /// Keeps track of the changes made to the todo list.
    changes: Vec<TodoAppEvent>,
    running: bool,
}

impl AppState {
    pub fn new() -> AppState {
        let todos = load(FILEPATH);
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

    pub fn undo(&mut self) {
        if let Some(event) = self.changes.pop() {
            match event {
                TodoAppEvent::AddTodoAt(_, index) => self.remove_todo(index),
                TodoAppEvent::ToggleComplete(index) => self.toggle_complete(index),
                TodoAppEvent::RemoveTodoAt(item, _) => self.add_todo(item),
            }
            self.changes.pop(); // We don't want to put the removing action on the change stack.
        }
    }

    pub fn is_running(&self) -> bool {
        self.running
    }

    pub fn quit(&mut self) {
        self.running = false;
        self.save(FILEPATH)
    }

    pub fn save(&self, path: &str) {
        use std::fs::File;
        use std::io::{BufWriter, Write};

        let file = match File::create(path) {
            Ok(file) => file,
            Err(msg) => {
                println!("Error: {}", msg);
                return;
            }
        };
        let mut file = BufWriter::new(file);

        for item in self.todos.iter() {
            writeln!(file, "{},{}", item.description, item.complete).unwrap();
        }
    }
}

fn load(path: &str) -> Vec<TodoItem> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    let mut items = Vec::new();
    let file = match File::open(path) {
        Ok(file) => file,
        Err(msg) => {
            println!("Error: {}\nUsing an empty todo list.", msg);
            return items;
        }
    };

    for line in BufReader::new(file).lines() {
        let line = line.unwrap();
        if line.contains(',') {
            let split: Vec<_> = line.split(',').collect();
            if split.len() != 2 {
                panic!("{} is not the expected format!", path);
            }
            let complete = split[1].trim().parse::<bool>().unwrap();
            items.push(TodoItem::new(split[0].trim(), complete));
        }
    }

    items
}
