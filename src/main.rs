mod app_state;
mod app_event;
mod todo_item;

use app_state::AppState;
use todo_item::TodoItem;

fn main() {
    let mut state = AppState::new();
    state.add_todo(TodoItem::new("Actually do some programming.", false));
    state.add_todo(TodoItem::new("Actually do other stuff.", false));
    state.remove_todo(0);
    state.replace_todo(0, TodoItem::new("Actually, do this instead.", false));

    println!("{:?}", state.get_todo_list());
    println!("{:?}", state.get_changes());
}