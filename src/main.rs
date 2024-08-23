use std::io;

fn main() {
    println!("Please enter your name:");

    let mut name = String::new();
    io::stdin().read_line(&mut name)
        .expect("Failed to read line");

    // Remove the newline character from the input
    let name = name.trim();

    let message = format!("Hello, {}!", name);
    println!("{}", message);
}