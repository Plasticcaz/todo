mod app_state;
mod app_event;
mod todo_item;

use app_state::AppState;
use todo_item::TodoItem;

fn main() {
    let state = AppState::new();
    let items: Vec<MenuItem<AppState>> = vec![
        MenuItem::new("Exit:", |state| state.quit()),
        MenuItem::new("Display todos:", display_todos),
        MenuItem::new("Add Todo:", add_todo),
        MenuItem::new("Toggle complete:", toggle_complete),
        MenuItem::new("Remove Todo:", remove_todo)
    ];
    let mut menu = Menu::new(state, items);

    while menu.state.is_running() {
        menu.display();
        menu.choose();
    }
}

fn remove_todo(state: &mut AppState) {
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
}

fn toggle_complete(state: &mut AppState) {
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
}

fn add_todo(state: &mut AppState) {
    // Query the user for a new Todo:
    let mut description = String::new();
    println!("Please write the task description:");

    if let Err(msg) = std::io::stdin().read_line(&mut description) {
        println!("{}", msg);
        return;
    }

    state.add_todo(TodoItem::new(description.trim(), false));
}

fn display_todos(state: &mut AppState) {
    for item in state.get_todo_list() {
        println!("\t{}", item);
    }
}

pub struct Menu<State> {
    // TODO(zac): We should really make this private and look into exposing a while_running in the menu.
    pub state: State,
    items: Vec<MenuItem<State>>,
}

impl<State> Menu<State> {
    pub fn new(state: State, items: Vec<MenuItem<State>>) -> Menu<State> {
        Menu {
            state,
            items,
        }
    }
    pub fn display(&self) {
        println!("\n\n\nTODO MENU:\nPick the number for the option you wish to select:");
        for (index, option) in self.items.iter().enumerate() {
            println!("\t{} - {}", index, option.description);
        }
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

pub struct MenuItem<State> {
    pub description: String,
    pub action: fn(&mut State),
}

impl<State> MenuItem<State> {
    pub fn new<S: Into<String>>(description: S, action: fn(&mut State)) -> MenuItem<State> {
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
