use crate::todo::load_todo::load_todo;

use std::fs;
use std::io;

pub fn update_todo() -> Result<(), Box<dyn std::error::Error>> {
    println!("Enter todo id to update");
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
    let mut found = false;
    for todo in todos.iter_mut() {
        if todo.id == id {
            todo.completed = true;
            found = true;
            break;
        }
    }

    if !found {
        println!("The todo is not found");
    }

    let serialized = serde_json::to_string(&todos)?;
    fs::write("todo.json", serialized)?;

    Ok(())
}
