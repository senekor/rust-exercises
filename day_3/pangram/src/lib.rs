//! This exerice was yoinked from Exercism (MIT licensed):
//! https://github.com/exercism/rust/tree/main/exercises/practice/pangram
//!
//! This exercise was modified to be more opinionated in how you should solve it.
//! The purpose is to deepen your understanding of iterators by writing your own collection.
//!
//! The general exerice instructions are:
//!
//! Determine if a sentence is a pangram.
//! A pangram is a sentence using every letter of the alphabet at least once.
//!
//! The alphabet used consists of ASCII letters `a` to `z` and is case insensitive.
//! Any characters which are not an ASCII letter should be ignored.

// -------------------------------- STEP 1 --------------------------------
//
// Define a type called `LetterSet` to store which letters are contained in a sentence.
// Whether a letter is contained could be a bool or a bit, and we need at least 26 of them.
// Recall the rules of the exerice: non-ascii characters are ignored and case doesn't matter.
//
// TODO

// -------------------------------- STEP 2 --------------------------------
//
// Turn your LetterSet into an idiomatic collection.
// Collections can be "collected into" by calling `.collect()` on an iterator.
// The collect method is powered by the `FromIterator` trait, so let's implement that.
// Documentation of the `FromIterator` trait:
// https://doc.rust-lang.org/stable/std/iter/trait.FromIterator.html
//
// In our case, we only need to be able to collect from an iterator over `char`.
//
// TODO

// -------------------------------- STEP 3 --------------------------------
//
// Naturally, we also want to be able to iterate over our collection.
//
// Recall that a vector and an iterator over a vector are different types,
// the latter can be acquired by calling `.into_iter()` on the former.
//
// Let's keep it simple here and implement Iterator directly on LetterSet.
//
// TODO

// -------------------------------- STEP 4 --------------------------------
//
// To solve the exercise, we need to know if all 26 letters are present in the set.
// Give your LetterSet a method `is_full` for this purpose.
//
// TODO

// -------------------------------- STEP 5 --------------------------------
//
// Determine whether a sentence is a pangram.
// This is now easy:
// Construct a LetterSet from the sentence and determine if it's full.
//
pub fn is_pangram(sentence: &str) -> bool {
    todo!("Is {sentence} a pangram?");
}

#[cfg(deactivated)] // remove this line to activate the tests
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn empty_set_is_empty() {
        let letter_set: LetterSet = "".chars().collect();
        let hash_set: HashSet<_> = letter_set.into_iter().collect();
        let expected: HashSet<_> = "".chars().collect();
        assert_eq!(hash_set, expected)
    }

    #[test]
    fn can_store_a_character() {
        let letter_set: LetterSet = "a".chars().collect();
        let hash_set: HashSet<_> = letter_set.into_iter().collect();
        let expected: HashSet<_> = "a".chars().collect();
        assert_eq!(hash_set, expected)
    }

    #[test]
    fn is_case_insensitive() {
        let letter_set: LetterSet = "A".chars().collect();
        let hash_set: HashSet<_> = letter_set.into_iter().collect();
        let expected: HashSet<_> = "a".chars().collect();
        assert_eq!(hash_set, expected)
    }

    #[test]
    fn non_ascii_is_ignored() {
        let letter_set: LetterSet = "ö".chars().collect();
        let hash_set: HashSet<_> = letter_set.into_iter().collect();
        let expected: HashSet<_> = "".chars().collect();
        assert_eq!(hash_set, expected)
    }

    #[test]
    fn can_store_multiple_characters() {
        let letter_set: LetterSet = ('h'..'s').collect();
        let hash_set: HashSet<_> = letter_set.into_iter().collect();
        let expected: HashSet<_> = ('h'..'s').collect();
        assert_eq!(hash_set, expected)
    }

    #[test]
    fn insertion_order_is_ignored() {
        let letter_set: LetterSet = ('h'..'s').rev().collect();
        let hash_set: HashSet<_> = letter_set.into_iter().collect();
        let expected: HashSet<_> = ('h'..'s').collect();
        assert_eq!(hash_set, expected)
    }

    #[test]
    fn duplicate_insertions_are_ignored() {
        let letter_set: LetterSet = ('h'..'p').chain('K'..'S').collect();
        let hash_set: HashSet<_> = letter_set.into_iter().collect();
        let expected: HashSet<_> = ('h'..'s').collect();
        assert_eq!(hash_set, expected)
    }

    #[test]
    fn is_full_when_full() {
        let letter_set: LetterSet = ('a'..='z').collect();
        assert!(letter_set.is_full())
    }

    #[test]
    fn is_not_full_when_not_full() {
        let letter_set: LetterSet = ('a'..'y').collect();
        assert!(!letter_set.is_full())
    }
}

#[test]
fn exercise_was_started() {
    let this_file_content = include_str!("lib.rs");
    assert!(
        this_file_content
            .lines()
            .all(|line| !line.starts_with("#[cfg(deactivated)]")),
        "
        ╭──────────────────────────────────────────────────────────────────────────╮
        │ remove the line starting with #[cfg(deactivated)] to activate the tests! │
        ╰──────────────────────────────────────────────────────────────────────────╯
        "
    )
}
