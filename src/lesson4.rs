//! Lesson 4 exercises: collections and iterators.
//!
//! The exercise tests are marked `#[ignore]` so normal `cargo test` and CI
//! stay green. Run them while you work with:
//!
//!     cargo test lesson4 -- --ignored
//!
//! Replace each `todo!()` with a real implementation until its test passes.
//! Once an exercise passes, delete its `#[ignore]` line.
#![allow(dead_code, unused_variables)]

use std::collections::HashMap;
use std::num::ParseIntError;

/// Worked example from the lesson: count how often each word appears,
/// ignoring case.
pub fn word_counts(text: &str) -> HashMap<String, usize> {
    let mut counts = HashMap::new();
    for word in text.split_whitespace() {
        *counts.entry(word.to_lowercase()).or_insert(0) += 1;
    }
    counts
}

/// Exercise 1: return the sum of the squares of the even numbers.
/// Write it as a single iterator chain: no `for` loop and no `mut`.
pub fn sum_of_squares_of_evens(nums: &[i32]) -> i32 {
    todo!("filter, map, sum")
}

/// Exercise 2: return the longest word, or `None` if there are no words.
/// Hint: `split_whitespace` and `max_by_key` with `str::len`.
pub fn longest_word(text: &str) -> Option<&str> {
    todo!("split, then find the longest")
}

/// Exercise 3: map each word length to the words of that length, keeping
/// the input order within each group.
/// Hint: `map.entry(len).or_default().push(word)`.
pub fn group_by_length<'a>(words: &[&'a str]) -> HashMap<usize, Vec<&'a str>> {
    todo!("build the map with entry")
}

/// Exercise 4: parse comma-separated integers such as "1,2,3".
/// One iterator chain ending in `collect()`. Any bad item makes the whole
/// result an `Err`.
pub fn parse_all(s: &str) -> Result<Vec<i32>, ParseIntError> {
    todo!("split, parse each item, collect into a Result")
}

#[cfg(test)]
mod tests {
    use super::*;

    // Worked example from the lesson. This already passes.

    #[test]
    fn word_counts_ignores_case() {
        let counts = word_counts("the cat and The dog and the bird");
        assert_eq!(counts["the"], 3);
        assert_eq!(counts["and"], 2);
        assert_eq!(counts.get("fish"), None);
    }

    // Exercises. Remove `#[ignore]` once each one passes.

    #[test]
    #[ignore = "exercise 1"]
    fn exercise1_sum_of_squares_of_evens() {
        assert_eq!(sum_of_squares_of_evens(&[1, 2, 3, 4]), 20);
        assert_eq!(sum_of_squares_of_evens(&[]), 0);
        assert_eq!(sum_of_squares_of_evens(&[1, 3, 5]), 0);
    }

    #[test]
    #[ignore = "exercise 2"]
    fn exercise2_longest_word() {
        assert_eq!(longest_word("the quick fox jumped"), Some("jumped"));
    }

    #[test]
    #[ignore = "exercise 2"]
    fn exercise2_longest_word_of_nothing_is_none() {
        assert_eq!(longest_word(""), None);
        assert_eq!(longest_word("   "), None);
    }

    #[test]
    #[ignore = "exercise 3"]
    fn exercise3_group_by_length() {
        let groups = group_by_length(&["a", "bb", "cc", "d"]);
        assert_eq!(groups.len(), 2);
        assert_eq!(groups[&1], vec!["a", "d"]);
        assert_eq!(groups[&2], vec!["bb", "cc"]);
    }

    #[test]
    #[ignore = "exercise 4"]
    fn exercise4_parse_all_ok() {
        assert_eq!(parse_all("1,2,3").unwrap(), vec![1, 2, 3]);
    }

    #[test]
    #[ignore = "exercise 4"]
    fn exercise4_parse_all_fails_on_bad_item() {
        assert!(parse_all("1,x,3").is_err());
    }
}
