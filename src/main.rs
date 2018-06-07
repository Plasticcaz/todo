mod app_event;
mod app_state;
mod menu;
mod todo_item;

use app_state::AppState;
use menu::{add_todo, remove_todo, toggle_complete, Menu, MenuItem};

fn main() {
    let state = AppState::new();
    let items: Vec<MenuItem> = vec![
        MenuItem::new("Exit:", AppState::quit),
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
