use crate::todo::{Todo, generate_id::generate_id, load_todo::load_todo};
use colored::*;
use std::fs;
use std::io;

pub fn add_todo() -> Result<(), Box<dyn std::error::Error>> {
    let mut tasks: Vec<Todo> = load_todo()?;
    println!("{}", "Enter your task text".cyan().bold());
    let mut user_text = String::new();

    io::stdin()
        .read_line(&mut user_text)
        .expect("Can't read line");

    let id = generate_id()?;
    tasks.push(Todo {
        id,
        text: user_text.trim().to_string(),
        completed: false,
    });

    let serialized = serde_json::to_string(&tasks)?;

    fs::write("todo.json", serialized)?;

    println!("Added task of id = {}", id);

    Ok(())
}
