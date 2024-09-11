//! Write a function that calculates the [digit sum](https://en.wikipedia.org/wiki/Digit_sum).
//!
//! Your users want to be able to supply both strings and normal integers to the function.
//! Design a trait that allows you to do that.

// You will need to change this function signature, such that it accepts
// more than just usize.
fn calculate_digit_sum(number: usize) -> usize {
    todo!()
}

#[cfg(deactivated)] // remove this line to activate the tests
mod tests {
    use super::*;

    #[test]
    fn integer() {
        assert_eq!(calculate_digit_sum(1235), 11);
    }

    #[test]
    fn string() {
        assert_eq!(calculate_digit_sum("1235"), 11);
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
