use std::env;

fn main() {
    // Use the first command-line argument as the name, or default to "world".
    let name = env::args().nth(1).unwrap_or_else(|| "world".to_string());
    println!("Hello, {name}!");
}
