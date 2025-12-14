use std::fs;
use std::io;

use crate::todo::load_todo::load_todo;

pub fn delete_todo() -> Result<(), Box<dyn std::error::Error>> {
    println!("Enter todo id to delete");
    let mut id = String::new();

    io::stdin().read_line(&mut id).expect("Can't read line");

    let id: i32 = match id.trim().parse() {
        Ok(d) => d,
        Err(_) => {
            println!("Please enter a number");
            return Ok(());
        }
    };

    let mut todos = load_todo()?;
    let before = todos.len();

    todos.retain(|t| t.id != id);

    if todos.len() == before {
        println!("Can't find the todo with id = {}", id);
    }

    let serialized = serde_json::to_string(&todos)?;
    fs::write("todo.json", serialized)?;
    println!("Deleted todo is successfull");
    Ok(())
}
