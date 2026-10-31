use std::io::{self, Write};

fn main() {
    let mut x = String::new();
    print!("Enter you name: ");
    let _ = io::stdout().flush();
    io::stdin()
        .read_line(&mut x)
        .expect("Failed to store your name");

    println!("Welcome, {x}");
}
