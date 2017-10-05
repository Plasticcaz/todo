mod app_state;
mod app_event;
mod todo_item;

use app_state::AppState;
use todo_item::TodoItem;

fn display_todos(state: &mut AppState) {
    for item in state.get_todo_list() {
        let marker = if item.complete {
            '✓'
        }
        else {
            '✕'
        };
        println!("\t{} - {}", marker, item.description);
    }
}

fn main() {
    let state = AppState::new();
    let items: Vec<MenuItem<AppState>> = vec![
        MenuItem::new("Exit:", |state| state.quit()),
        MenuItem::new("Display todos:", display_todos),
        MenuItem::new("Add Todo:", |state| state.add_todo(TodoItem::new("Allow user to input their own todos.", false))),
    ];
    let mut menu = Menu::new(state, items);

    while menu.state.is_running() {
        menu.display();
        menu.choose();
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
        println!("TODO MENU:\nPick the number for the option you wish to select:");
        for (index, option) in self.items.iter().enumerate() {
            println!("\t{} - {}", index, option.description);
        }
    }

    /// Have the user choose a menu item, and execute the action associated with that 
    /// item.
    pub fn choose(&mut self) {
        let mut choice = String::new();
        if std::io::stdin().read_line(&mut choice).is_ok() {
            let choice = choice.trim().parse::<usize>().unwrap();
            if let Some(item) = self.items.get(choice) {
                let perform_action = item.action;
                perform_action(&mut self.state);
            }
            else {
                println!("Please select one of the options provided.")
            }
        }
        else {
            println!("Failed to read choice.");
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
