mod todo;
use colored::*;
use figlet_rs::FIGfont;
use std::io;

use todo::{Todo, add_todo};
fn main() {
    let font = FIGfont::standard().unwrap();
    let figure = font.convert("Todo List Cli").unwrap();
    println!("{}", figure.to_string().cyan().bold());

    println!("{}", "Choose your option".green().bold());
    println!("1. Add todo");
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("can't read line");
    let choice: u32 = match choice.trim().parse() {
        Ok(d) => d,
        Err(_) => {
            println!("Please enter a number");
            return;
        }
    };
}
