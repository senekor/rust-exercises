# Day 5

Here's a checklist for this week's tasks:

## Practice

There is a library called `badlib` already prepared.
We will incrementally build a CI pipeline and improve this library as we go along.

### Creating a "Hello World!" CI pipeline

Create a file `.github/workflows/ci_workflow.yml` in the **root of your repository**.
The specific path is important (except the name "ci_workflow"), because GitHub will automatically pick it up for its automation system ("GitHub Actions").
Copy the following content into this file:

```yml
name: CI Workflow
on: push

jobs:
  ci:
    name: CI Job
    runs-on: ubuntu-latest
    steps:
      - name: My first CI step
        run: echo "Hello CI! Let's hope I get a green checkmark."
```

A brief explanation skipping many details:

GitHub Actions are organized in workflows, jobs and steps.
We're only going to have one workflow and one job, no need to worry about those.
Within our one job, we can run several steps to sequentially execute all the checks we want.

The file `ci_workflow.yml` defines our workflow.
Within the file, we give the workflow a name with `name: CI Workflow`.
The following line is `on: push`, which means this workflow will be executed every time you `git push` to your repository. (Many other automation triggers are available, but we won't need them.)

Next comes a list of jobs where we have a single job specified with `ci:`.
We also give it a custom name with `name: CI Job`.
Then we specify the environment within which our job runs: `runs-on: ubuntu-latest`.
You usually only mess with this when you need to ensure cross-platform compatibility.

Lastly, we specify a list of steps below `steps:`.
As we go along, we will simply extend this list with more steps.
Every step has an optional custom `name`.
We choose the command(s) to run with `run:`.
If all commands in all our steps pass, our CI pipeline will succeed!

Recall that the file format is YAML, which means whitespace / indentation is significant!

Now, let's commit this file and push it to GitHub.

If all goes well, you should find the workflow run on GitHub, on the page of your repository, under the tab "Actions".
Click on the workflow run, you should see the single job we defined.
Click on that as well.
Now you should see the list of steps that were executed.
"Set up job" and "Complete job" are steps from GitHub, they're always there.
Click on "My first CI step" and you should see the output of your `echo` command!

### Enforce standard formatting

Let's write our first CI check with substance.
Standardized formatting is a great way to reduce mental load when working in a team.
That's why the Rust toolchain ships which `rustfmt` by default. A great quote about the formatter of Go, which applies to Rust's `rustfmt` as well:

> `gofmt`'s style is no one's favorite, yet `gofmt` is everyone's favorite.

Let's replace our dummy step with this one:

```yml
- name: Ensure standard formatting
  run: cargo fmt --check
```

If you push this version of the workflow, it will fail.
Feel free to try it yourself.
The error message starts like this:

> `cargo metadata` exited with an error: error: could not find `Cargo.toml` in `/home/runner/work/rw-test/rw-test` or any parent directory

The reason is that GitHub Actions by default don't even have access to your own code!
That's why almost every CI pipeline you'll see in the wild starts with a step to checkout the code.
Thirdy-party actions can be imported or "used", which is convenient.
The action to checkout your code is provided by GitHub itself.

Our next attempt looks like this:

```yml
steps:
  - uses: actions/checkout@v4 # <-- this line is new
  - name: Ensure standard formatting
    run: cargo fmt --check
```

Commit and push this new workflow.
This time, you should get actual output by `rustfmt`, telling you how to fix the formatting in `badlib`:

```
Diff in /home/runner/work/rw-test/rw-test/day_5/badlib/src/lib.rs at line 5:
// ... diff ...
Error: Process completed with exit code 1.
```

Note that this only works because `rustup` is installed by default in every GitHub Actions runner.
That means you always have access to a default toolchain.
There are community-built Actions you can import for advanced caching of your build artifacts, making your CI even faster.
We won't look at them here.

Fix the formatting locally by running `cargo fmt`, commit and push the fix and make sure your CI now passes again.

### Improve code quality with a linter

Most popular languages have one or several _linters_ and static-analysis tools to help you improve your code.
They also make code review much more productive, since reviewers can focus on the more difficult aspects of writing good software which cannot be analyzed reliably by a program.

The Rust compiler itself already has a couple of lints that can be activated separately.
However, the big player is `clippy`.
It has a large number of advanced lints ranging over the topics of style, performance, complexity and correctness.
Some of them provide auto-fix suggestions which integrate with `rust-analyzer`.

Let's run `clippy` over our code and see what it has to say!
Add the following CI step to the existing ones.

```yml
- name: Check code quality with clippy
  run: cargo clippy -- --deny warnings
```

Commit and push this new CI pipeline and observe the errors generated by clippy.
Now, fix those errros and make CI pass again.

Note that in order for clippy to succeed, the code needs to compile in the first place.
That's why we don't need a separate step for that.
If, for some reason, you want to check compilation without using clippy, the command to run in CI is `cargo check`.

Now, let's say our library is super-duper important and we want to enable even more lints.
This can be done nicely in the project manifest, so open `badlib/Cargo.toml`.
Copy-paste the following at the end:

```toml
[lints.rust]
missing_docs = "deny"

[lints.clippy]
cast_possible_truncation = "deny"
```

One of the above disabled-by-default lints comes from the Rust compiler itself, so it goes under the section `[lints.rust]`.
Lints that come from clippy, which are most of them, go under `[lints.clippy]`.
You can find more clippy lints to enable [here](https://rust-lang.github.io/rust-clippy/stable/index.html).

Commit and push these additions to `Cargo.toml`.
As always, make CI fail first, then fix it in a separate step.

### Running tests

Tests are an extremely important tool to reduce the number of bugs in the software we write.
But they aren't always fun.
Dealing with broken tests because of your coworkers is not fun... and neither is being responsible for breaking your coworker's tests!
It happens to the best of us.

Let's run our tests CI as well, so we can be sure there aren't any _known_ bugs merged into the main branch.

```yml
- name: Run tests
  run: cargo test
```

You know the drill by now.
Break CI first, before fixing the tests.

### Publishing pre-built binaries

Imagine you update the version number in your `Cargo.toml`, push that change to GitHub with a corresponding tag and a pre-built binary of you application is automatically generated and made available for download.
Wouldn't that be amazing?
Imagine no more, we're gonna do it right now.

First, let's create a new application to ship to our users:

```sh
cd rust-exercises/day_5
cargo new shippable
```

Change the text in the `println!` statement to something personal.

Next, you're gonna need to have `cargo-dist` installed, which is gonna do most of the heavy-lifting for us.
This depends on your platform and setup, please refer to the [install page](https://opensource.axo.dev/cargo-dist/book/install.html).
My personal preference is `cargo binstall cargo-dist`.

There's one thing we have to prepare for `cargo-dist` to be able to do its job.
Every `Cargo.toml` in the workspace must have the repository key defined:

- For the `Cargo.toml` at the root of the repository (the workspace manifest), add it below the `members` key:

  ```toml
  members = ["day_?/*"]
  repository = "https://github.com/<YOUR_GITHUB_USERNAME>/rust-exercises"
  ```

- For all other `Cargo.toml`s (the individual project manifests), add it below the `edition` key:

  ```toml
  edition = "2021"
  repository = "https://github.com/<YOUR_GITHUB_USERNAME>/rust-exercises"
  ```

Now run `cargo dist init --yes`, which will generate a release configuration with decent default values for you.
There should be some additions to `Cargo.toml` in the root of your repository as well as a new file `.github/workflows/release.yml`.

Commit these changes and push them to enable the new workflow.

Now, for the last part, we're just going to create a new release and watch what happens.

Update the `version` key, e.g. to `0.1.1`, in the `Cargo.toml` of the binary you want to ship.
(Consistent with the previous example, that would be `day_5/shippable`.)
Consider updating the `println!` statement as well, so you can be sure the correct version was built.
Commit and push these changes.

To trigger `cargo-dist`, tag your latest commit with the version you just updated to and push the tag:

```sh
git tag v0.1.1
git push --tags
```

That's it!
Keep an eye on your GitHub Actions tab, the release workflow should be triggered.
Once it is done, you can find a new release on the front page of your GitHub repo.
It should contain downloadable binaries for all common platforms.

How cool is that! 😎

## Homework

- [ ] skim [blessed.rs](https://blessed.rs) from top to bottom.
      This will give you an overview of the highest-quaility and most important libraries available.

## Final Project

Once you're done with this week's practice tasks, you can get started with the [final project](https://senekor.github.io/paekli-rs)!
