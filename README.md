# Rust Exercises

This repository contains the exercises that go along with my [Rust workshop][workshop-repo].

Below, you'll find additional learning resources that you may want to keep in reach while learning Rust.
The list is intentionally kept short and opinionated.

Get started with [day_1/README.md](day_1/README.md).

## Learning resources

- [The Book][the-book], for learning methodically
- [Rust By Example][rust-by-example], for looking things up

## Practice resources

- language & syntax basics: [rustlings]
- Rust-specific exercises with provided tests: [Exercism][exercism]\
  Setup recommendation:
  [Use the CLI to work locally](https://exercism.org/docs/using/solving-exercises/working-locally).
  You can keep your solutions in this repo by configuring the workspace:
  ```sh
  exercism configure --workspace rust-exercises/exercism
  ```
- general programming exercises (text-in, text-out): [Advent of Code][advent-of-code]
  Setup recommendation:
  Simply run `cargo new` for each exercise and copy-paste the input data.
  You can do it right in this repo for convenience, if you like. 

## Ecosystem Wegweiser

- community-driven shortlist of high-quality libraries: [blessed.rs][blessed-rs]
- library documentation: [docs.rs][docs-rs]
  - handy shortcut: `docs.rs/<library-name>`
  - example for the `winnow` crate: [docs.rs/winnow][docs-winnow]
- community-driven newsletter: [This Week in Rust][this-week-in-rust]

## Slides

The slides for the workshop are NOT intended as reference material, but you can find them [here][slides] anyway.

<!-- references -->

[workshop-repo]: https://github.com/senekor/rust-workshop
[the-book]: https://doc.rust-lang.org/book/
[rust-by-example]: https://doc.rust-lang.org/rust-by-example/
[rustlings]: https://rustlings.cool/
[exercism]: https://exercism.org/tracks/rust
[advent-of-code]: https://adventofcode.com/
[blessed-rs]: https://blessed.rs/crates
[docs-rs]: https://docs.rs/
[docs-winnow]: https://docs.rs/winnow
[this-week-in-rust]: https://this-week-in-rust.org/
[slides]: https://senekor.github.io/rust-workshop/
