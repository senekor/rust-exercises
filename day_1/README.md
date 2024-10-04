# Day 1

Here's a checklist for this week's tasks:

## Practice

- [ ] Create a repository from [the template][exercise-repo].
- [ ] [Install the Rust toolchain][install-rust].
- [ ] Setup your editor for Rust development.
      If you're using Visual Studio Code, simply install the recommended extensions.
      For other editors, make sure the following is working:
  - LSP ([rust-analyzer])
  - debugging support
- [ ] Configure your LSP to use Rust's official linter, clippy.
  - [Instructions for Visual Studio Code][vscode-clippy]
  - [ ] [`hello-clippy`](hello-clippy/src/lib.rs) contains linter warnings.
        Confirm that your editor shows them to you, then fix them.
- [ ] Setup [rustlings]:
  - [ ] run `cargo install rustlings`
  - [ ] in the root of your repo, run `rustlings init`
        This will create the `rustlings` folder containing all the exercises.
        (If you want to check their initial state into version control, now's a good time.)
- Solve rustlings exercises (run the command `rustlings` inside the `rustlings/` folder).
  - [ ] intro
  - [ ] variables
  - [ ] if
  - [ ] functions
  - [ ] quiz 1
  - [ ] primitive types
  - [ ] vectors
  - [ ] move semantics (== ownership)

## Homework

The purpose here is to get you comfortable reading documentation and make you aware of some useful things.
Don't get bogged down in the details.
Read until you are satisfied and make a mental note that these things exist.

- [ ] Check out "Rust By Example"
  - [Nesting and labels](https://doc.rust-lang.org/rust-by-example/flow_control/loop/nested.html)
- [ ] standard library documentation
  - [`str::chars`](https://doc.rust-lang.org/stable/std/primitive.str.html#method.chars)
  - [`str::split`](https://doc.rust-lang.org/stable/std/primitive.str.html#method.split)
  - [`str::lines`](https://doc.rust-lang.org/stable/std/primitive.str.html#method.lines)
  - [`str::parse`](https://doc.rust-lang.org/stable/std/primitive.str.html#method.parse)
  - [`fmt`](https://doc.rust-lang.org/stable/std/fmt/index.html) (module-level documentation of string formatting syntax, used in `println!`)
  - [`env::args`](https://doc.rust-lang.org/stable/std/env/fn.args.html)
  - [`slice::reverse`](https://doc.rust-lang.org/stable/std/primitive.slice.html#method.reverse)
  - [`slice::sort`](https://doc.rust-lang.org/stable/std/primitive.slice.html#method.sort)
  - [`slice::windows`](https://doc.rust-lang.org/stable/std/primitive.slice.html#method.windows)
  - [`dbg!`](https://doc.rust-lang.org/stable/std/macro.dbg.html)
  - [`include_str!`](https://doc.rust-lang.org/stable/std/macro.include_str.html)
  - [`todo!`](https://doc.rust-lang.org/stable/std/macro.todo.html)

## Optional exercises

If you have time and enthusiasm to spare, you can solve some of these exercises.
You are encouraged to ask your workshop organizer for a code review!
Feedback will focus on adding information, pointing out different ways of doing things and discussing trade-offs. (no nitpicking / perfectionism)

These exercises were selected because you already learned about everything required to solve them.
However, a performant and or elegant solution might still be out of reach!
So, keep your perfectionism in check for now.
You can always revisit these exercises later to refactor them.

This is not a checklist, but rather an "allow-list"!
Just start with one that appeals to you and do as many as you like.
Exercism focuses on exercises that are designed to push you to use specific language features.
Advent of Code on the other hand is more free-form problem solving.

Make sure you finished the standard library reading, some of those items might come in handy 😉

- [Exercism] (setup recommendations in the [top-level readme](../README.md))
  - [Grains](https://exercism.org/tracks/rust/exercises/leap): bit fiddling
  - [Leap](https://exercism.org/tracks/rust/exercises/leap): boolean operators
  - [Prime Factors](https://exercism.org/tracks/rust/exercises/nth-prime): arithmetic, vectors
  - [Proverb](https://exercism.org/tracks/rust/exercises/proverb): slices, string manipulation
  - [Matching Brackets](https://exercism.org/tracks/rust/exercises/matching-brackets): vectors, characters
  - [Sieve](https://exercism.org/tracks/rust/exercises/sieve) (of Eratosthenes): arithmetic, vectors
  - [Secret Handshake](https://exercism.org/tracks/rust/exercises/secret-handshake): bit fiddling, vectors
- [Advent of Code] (setup recommendations in the [top-level readme](../README.md))
  - [2015: day 1](https://adventofcode.com/2015/day/1)
  - [2015: day 2](https://adventofcode.com/2015/day/2)
  - [2017: day 1](https://adventofcode.com/2017/day/1)
  - [2019: day 1](https://adventofcode.com/2019/day/1)
  - [2019: day 2](https://adventofcode.com/2019/day/2)
  - [2020: day 1](https://adventofcode.com/2020/day/1)
  - [2021: day 1](https://adventofcode.com/2021/day/1)
  - [2021: day 2](https://adventofcode.com/2021/day/2)
  - [2022: day 1](https://adventofcode.com/2022/day/1)
  - [2022: day 2](https://adventofcode.com/2022/day/2)

[install-rust]: https://www.rust-lang.org/tools/install
[exercise-repo]: https://github.com/senekor/rust-exercises
[rust-analyzer]: https://rust-analyzer.github.io/
[vscode-clippy]: https://code.visualstudio.com/docs/languages/rust#_linting
[helix-website]: https://helix-editor.com/
[rustlings]: https://github.com/rust-lang/rustlings
[Exercism]: https://exercism.org/tracks/rust
[Advent of Code]: https://adventofcode.com
