//! Lesson 5 exercises: traits, generics and lifetimes.
//!
//! The exercise tests are marked `#[ignore]` so normal `cargo test` and CI
//! stay green. Run them while you work with:
//!
//!     cargo test lesson5 -- --ignored
//!
//! Replace each `todo!()` with a real implementation until its test passes.
//! Once an exercise passes, delete its `#[ignore]` line.
#![allow(dead_code, unused_variables)]

/// Worked example from the lesson: the largest item of a slice.
pub fn largest<T: PartialOrd>(items: &[T]) -> Option<&T> {
    let mut best = items.first()?;
    for item in items {
        if item > best {
            best = item;
        }
    }
    Some(best)
}

// ---------------------------------------------------------------------
// Exercise 1: a trait with a default method
// ---------------------------------------------------------------------

pub trait Animal {
    fn name(&self) -> String;
    fn sound(&self) -> String;

    /// Default method: should return e.g. "Rex says woof"
    /// (the name, then " says ", then the sound).
    fn speak(&self) -> String {
        todo!("combine name() and sound()")
    }
}

pub struct Dog {
    pub name: String,
}

impl Animal for Dog {
    fn name(&self) -> String {
        self.name.clone()
    }
    fn sound(&self) -> String {
        "woof".to_string()
    }
}

pub struct Cat {
    pub name: String,
}

/// Implement `Animal` for `Cat`. Its sound is "meow".
impl Animal for Cat {
    fn name(&self) -> String {
        todo!("return the cat's name")
    }
    fn sound(&self) -> String {
        todo!("return the cat's sound")
    }
}

// ---------------------------------------------------------------------
// Exercise 2: a generic function
// ---------------------------------------------------------------------

/// Return the smallest item, or `None` for an empty slice.
/// It is the mirror image of `largest` above.
pub fn smallest<T: PartialOrd>(items: &[T]) -> Option<&T> {
    todo!("mirror largest")
}

// ---------------------------------------------------------------------
// Exercise 3: a generic type
// ---------------------------------------------------------------------

pub struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack { items: Vec::new() }
    }

    pub fn push(&mut self, item: T) {
        self.items.push(item);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }

    /// Look at the top item without removing it.
    pub fn peek(&self) -> Option<&T> {
        todo!("borrow the last item")
    }

    pub fn is_empty(&self) -> bool {
        todo!("true when there are no items")
    }
}

impl<T> Default for Stack<T> {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------
// Exercise 4: a lifetime
// ---------------------------------------------------------------------

/// Return the first whitespace-separated word, or "" if there is none.
/// The result borrows from `text`, which is what `'a` says.
///
/// Experiment afterwards: add a second parameter `other: &str` and see
/// why the compiler needs `'a` once there are two input references.
#[allow(clippy::needless_lifetimes)] // kept explicit on purpose for the lesson
pub fn first_word<'a>(text: &'a str) -> &'a str {
    todo!("split_whitespace and take the first, or \"\"")
}

#[cfg(test)]
mod tests {
    use super::*;

    // Worked example from the lesson. This already passes.

    #[test]
    fn largest_of_several_types() {
        assert_eq!(largest(&[3, 9, 2]), Some(&9));
        assert_eq!(largest(&["pear", "apple", "zebra"]), Some(&"zebra"));
        assert_eq!(largest::<i32>(&[]), None);
    }

    // Exercises. Remove `#[ignore]` once each one passes.

    #[test]
    #[ignore = "exercise 1"]
    fn exercise1_default_speak_for_dog() {
        let d = Dog {
            name: "Rex".to_string(),
        };
        assert_eq!(d.speak(), "Rex says woof");
    }

    #[test]
    #[ignore = "exercise 1"]
    fn exercise1_cat_implements_animal() {
        let c = Cat {
            name: "Tom".to_string(),
        };
        assert_eq!(c.speak(), "Tom says meow");
    }

    #[test]
    #[ignore = "exercise 1"]
    fn exercise1_trait_objects_in_a_list() {
        let pets: Vec<Box<dyn Animal>> = vec![
            Box::new(Dog {
                name: "Rex".to_string(),
            }),
            Box::new(Cat {
                name: "Tom".to_string(),
            }),
        ];
        let spoken: Vec<String> = pets.iter().map(|p| p.speak()).collect();
        assert_eq!(spoken, vec!["Rex says woof", "Tom says meow"]);
    }

    #[test]
    #[ignore = "exercise 2"]
    fn exercise2_smallest() {
        assert_eq!(smallest(&[3, 1, 2]), Some(&1));
        assert_eq!(smallest(&[1.5, 0.5]), Some(&0.5));
        assert_eq!(smallest(&["pear", "apple", "zebra"]), Some(&"apple"));
        assert_eq!(smallest::<i32>(&[]), None);
    }

    #[test]
    #[ignore = "exercise 3"]
    fn exercise3_peek_and_is_empty() {
        let mut s = Stack::new();
        assert!(s.is_empty());
        assert_eq!(s.peek(), None);
        s.push("a");
        s.push("b");
        assert!(!s.is_empty());
        assert_eq!(s.peek(), Some(&"b"));
        assert_eq!(s.pop(), Some("b")); // peek must not have removed it
        assert_eq!(s.peek(), Some(&"a"));
    }

    #[test]
    #[ignore = "exercise 4"]
    fn exercise4_first_word() {
        assert_eq!(first_word("hello world"), "hello");
        assert_eq!(first_word("  padded  text"), "padded");
    }

    #[test]
    #[ignore = "exercise 4"]
    fn exercise4_first_word_of_nothing() {
        assert_eq!(first_word(""), "");
        assert_eq!(first_word("   "), "");
    }
}
