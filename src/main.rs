mod app_state;
mod app_event;
mod todo_item;
mod menu;

use app_state::AppState;
use menu::{Menu, MenuItem, display_todos, add_todo, toggle_complete, remove_todo};

fn main() {
    let state = AppState::new();
    let items: Vec<MenuItem<AppState>> = vec![
        MenuItem::new("Exit:", AppState::quit),
        MenuItem::new("Display todos:", display_todos),
        MenuItem::new("Add Todo:", add_todo),
        MenuItem::new("Toggle complete:", toggle_complete),
        MenuItem::new("Remove Todo:", remove_todo),
        MenuItem::new("Undo Change:", AppState::undo),
    ];
    let mut menu = Menu::new(state, items);

    while menu.state.is_running() {
        menu.display();
        menu.choose();
    }
}
