use std::env;

mod lesson2;

/// Builds the greeting, defaulting to "world" when no name is given.
fn greeting(name: Option<&str>) -> String {
    format!("Hello, {}!", name.unwrap_or("world"))
}

fn main() {
    let name = env::args().nth(1);
    println!("{}", greeting(name.as_deref()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greets_given_name() {
        assert_eq!(greeting(Some("Charles")), "Hello, Charles!");
    }

    #[test]
    fn defaults_to_world() {
        assert_eq!(greeting(None), "Hello, world!");
    }
}
