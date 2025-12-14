use crate::todo::{
    add_todo::add_todo, delete_todo::delete_todo, show_todo::show_todo, update_todo::update_todo,
};

pub enum TodoOptions {
    Add,
    Show,
    Complete,
    Delete,
}

impl TodoOptions {
    pub fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            TodoOptions::Add => add_todo(),
            TodoOptions::Show => show_todo(),
            TodoOptions::Complete => update_todo(),
            TodoOptions::Delete => delete_todo(),
        }
    }
}
