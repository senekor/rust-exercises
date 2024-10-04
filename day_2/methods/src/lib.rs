//! Refactor the following program to use methods instead of plain functions.

#![allow(unused)]
#![deny(clippy::use_self)]

struct Bee {
    nectar: u32,
}

fn new_bee() -> Bee {
    Bee { nectar: 0 }
}

fn collect_nectar(bee: &mut Bee) {
    bee.nectar += 1
}

/// check how much nectar the bee is carrying
fn get_nectar(bee: &Bee) -> u32 {
    bee.nectar
}

/// The bee won't exist anymore after this function call 😢
/// Returns all the nectar the bee was carrying at the time of death.
fn die_for_the_glory_of_the_queen(bee: Bee) -> u32 {
    bee.nectar
}

#[cfg(deactivated)] // remove this line to activate the tests
mod tests {
    use super::*;

    #[test]
    fn can_create_bee() {
        let b = Bee::new();
        assert_eq!(b.get_nectar(), 0, "new bees start with zero nectar")
    }

    #[test]
    fn can_collect_nectar() {
        let mut b = Bee::new();
        b.collect_nectar();
        b.collect_nectar();
        assert_eq!(b.get_nectar(), 2, "bees collect one nectar at a time")
    }

    #[test]
    fn can_die_gloriously() {
        let mut b = Bee::new();
        b.collect_nectar();
        b.collect_nectar();
        b.collect_nectar();
        assert_eq!(
            b.die_for_the_glory_of_the_queen(),
            3,
            "bees return all remaining nectar upon death"
        )
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
