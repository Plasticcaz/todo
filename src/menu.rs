use app_state::AppState;
use todo_item::TodoItem;
use std;

pub fn remove_todo(state: &mut AppState) {
    clear_screen();
    if state.get_todo_list().len() == 0 {
        return;
    }
    // Print out all the options.
    for (index, item) in state.get_todo_list().iter().enumerate() {
        println!("\t{}. {}", index, item);
    }
    let mut choice = read_usize();
    while let Err(msg) = choice {
        println!("{}", msg);
        choice = read_usize();
    }
    state.remove_todo(choice.unwrap());
    println!();
}

pub fn toggle_complete(state: &mut AppState) {
    clear_screen();
    if state.get_todo_list().len() == 0 {
        return;
    }
    // Print out all the options.
    for (index, item) in state.get_todo_list().iter().enumerate() {
        println!("\t{}. {}", index, item);
    }

    let mut choice = read_usize();
    while let Err(msg) = choice {
        println!("{}", msg);
        choice = read_usize();
    }
    state.toggle_complete(choice.unwrap());
    println!();
}

pub fn add_todo(state: &mut AppState) {
    clear_screen();
    // Query the user for a new Todo:
    let mut description = String::new();
    println!("Please write the task description:");

    if let Err(msg) = std::io::stdin().read_line(&mut description) {
        println!("{}", msg);
        return;
    }

    state.add_todo(TodoItem::new(description.trim(), false));
    println!();
}

fn display_todos(state: &AppState) {
    clear_screen();
    println!("Todo:");
    let items = state.get_todo_list();
    if items.is_empty() {
        println!("There are no todos currently stored.");
    }

    for item in items { 
        println!("\t{}", item);
    }
    println!();
}

pub struct Menu {
    pub state: AppState,
    items: Vec<MenuItem>,
}

impl Menu {
    pub fn new(state: AppState, items: Vec<MenuItem>) -> Menu {
        Menu {
            state,
            items,
        }
    }
    pub fn display(&self) {
        display_todos(&self.state);
        println!("Main Menu:\nPick the number for the option you wish to select:");
        for (index, option) in self.items.iter().enumerate() {
            println!("\t{} - {}", index, option.description);
        }
        println!();
    }

    /// Have the user choose a menu item, and execute the action associated with that 
    /// item.
    pub fn choose(&mut self) {
        let choice = match read_usize() {
            Ok(choice) => choice,
            Err(msg) => {
                println!("{}", msg);
                return;
            }
        };

        if let Some(item) = self.items.get(choice) {
            let perform_action = item.action;
            perform_action(&mut self.state);
        }
        else {
            println!("Please select one of the options provided.")
        }
    }
}

pub struct MenuItem {
    pub description: String,
    pub action: fn(&mut AppState),
}

impl MenuItem {
    pub fn new<S: Into<String>>(description: S, action: fn(&mut AppState)) -> MenuItem {
        let description = description.into();
        MenuItem {
            description,
            action
        }
    }
}

fn read_usize() -> Result<usize, String> {
    let mut choice = String::new();
    if let Err(msg) = std::io::stdin().read_line(&mut choice) {
        return Err(format!("{}", msg));
    }
    choice.trim().parse::<usize>().map_err(|err| format!("{}", err))
}

fn clear_screen() {
    println!("\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n");
}