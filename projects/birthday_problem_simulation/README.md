# birthday_problem_simulation

This small Rust project runs a Monte Carlo simulation of the classic "birthday problem": estimate the probability that, in a group of N people, at least two share the same birthday.

This project is intentionally compact and meant as a learning exercise and demonstration of Rust basics and random sampling.

## Features

- Fast Monte Carlo simulation using the `rand` crate (version 0.9 as declared in Cargo.toml).
- Configurable group size and number of trials (see source for CLI / constants).

## Requirements

- Rust toolchain (rustup + cargo). Install: https://www.rust-lang.org/tools/install

## Run

From the project root (the directory that contains `Cargo.toml`):

```bash
cargo run --release
```

If the program accepts arguments, check the source for details; otherwise edit the constants in `src/main.rs` to change group size or trials.

## Test

No unit tests are provided by default. To run any tests if added:

```bash
cargo test
```

## Notes

- Dependency: `rand = "0.9"` (declared in Cargo.toml).
- The output is probabilistic — results will vary between runs. Use a large number of trials for a stable estimate.
