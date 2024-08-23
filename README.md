# Hello, World! 🌍

Welcome to my Rust project! 🦀

## About

I'm on a mission to learn Rust, the programming language that's as fast as a cheetah and as safe as a seatbelt. 🚀

## Why Rust?

Because who doesn't want to write code that's both **blazing fast** and **memory safe**? Plus, the Rustacean mascot is just too cute to resist. 🦀❤️

## What to Expect

- Lots of `println!("Hello, World!");` because, let's face it, that's the first step to world domination. 🌎
- A few (or many) compiler errors. But hey, no pain, no gain, right? 💪
- A sense of accomplishment when things finally compile. 🎉

## Join Me

Feel free to join me on this wild ride. Contributions, suggestions, and words of encouragement are always welcome! 😄

Happy coding! 🚀

## Rust Examples

### Variables and Mutability

```rust
fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}


fn main() {
    let result = add(5, 3);
    println!("The sum is: {}", result);
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}


fn main() {
    let number = 7;

    if number < 5 {
        println!("The number is less than 5");
    } else {
        println!("The number is greater than or equal to 5");
    }
}

use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let file_result = File::open("hello.txt");

    let file = match file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let user1 = User {
        username: String::from("someone"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
        active: true,
    };

    println!("Username: {}", user1.username);
    println!("Email: {}", user1.email);
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let msg = Message::Move { x: 10, y: 20 };

    match msg {
        Message::Quit => println!("Quit"),
        Message::Move { x, y } => println!("Move to x: {}, y: {}", x, y),
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => println!("Change color to red: {}, green: {}, blue: {}", r, g, b),
    }
}

```



