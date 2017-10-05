use std::fmt;

#[derive(Debug, Clone)]
pub struct TodoItem {
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

impl fmt::Display for TodoItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let marker = if self.complete {
            '✓'
        }
        else {
            '✕'
        };
        write!(f, "{} - {}", marker, self.description)
    }
}