# aureate

My personal Rust learning repository, containing chapter-based exercises,
learning progress notes, and Exercism practice under `rust-learning/`.

It includes small Cargo projects and standalone Rust programs. Some exercises
are unfinished and kept here as part of the learning process.

## Repository structure

```text
rust-learning/
├── Chapter_1/                  # Getting started
│   ├── hello_world/            # First program, compiled with rustc
│   └── hello_cargo/            # Hello, world! using Cargo
├── Chapter_2/                  # Guessing game
│   ├── guessing_game/          # Working number-guessing game
│   └── guessing_game_starter/  # Original starter, still prints Hello, world!
├── Chapter_3/                  # Common programming concepts
│   ├── common_concepts/        # Fundamentals practice using Cargo
│   ├── Variables/src/bin/      # Standalone practice programs
│   │   ├── coins.rs
│   │   ├── practice.rs
│   │   └── types.rs
│   ├── functions.rs            # Empty placeholder for further practice
│   ├── PROGRESS.md
│   └── FINISH_CHAPTER_3.md
├── Chapter_4/                  # Ownership and borrowing
│   ├── example1.rs             # Scope
│   ├── example2.rs             # Copying and cloning
│   ├── example3.rs             # Ownership transfer through a function
│   ├── example4.rs             # Shared and mutable references
│   ├── example5.rs             # Mutation through dereferencing
│   └── PROGRESS.md
├── Chapter_5/                  # Structs: work in progress
│   └── struct1.rs
└── Exercism/                   # Standalone practice implementations
    ├── Hello.rs
    ├── TwoLeap.rs
    ├── ThreeSquare.rs
    ├── FourRaindrops.rs
    └── FiveGrains.rs
```

`common_concepts` contains practice with variables, mutability, shadowing,
constants, data types, tuples, arrays, functions, expressions, conditions, and
loops. The smaller programs in `Variables/` practice mutability, shadowing,
arithmetic, and data types.

The Exercism programs cover Hello World, Leap, Difference of Squares,
Raindrops, and Grains. They use example inputs in `main` and do not include
Exercism test suites. The tree above omits build output and Cargo lockfiles.

## Prerequisites

- Rust and Cargo with support for the Rust 2024 edition used by the Cargo projects.
- A POSIX-compatible shell for the commands below, such as Bash on Linux or macOS.

Check that the tools are available:

```sh
rustc --version
cargo --version
```

## Running the exercises

Run all commands from the repository root. Each Cargo project is independent;
there is no root Cargo workspace. Use `cargo run --manifest-path` for projects
with a `Cargo.toml`, and `rustc` for standalone files. The guessing game's first
build may need internet access to download its dependencies.

Standalone examples below write executables into ignored `target/` directories.

### Chapter 1: Hello

```sh
mkdir -p rust-learning/Chapter_1/hello_world/target
rustc rust-learning/Chapter_1/hello_world/main.rs -o rust-learning/Chapter_1/hello_world/target/main
./rust-learning/Chapter_1/hello_world/target/main

cargo run --manifest-path rust-learning/Chapter_1/hello_cargo/Cargo.toml
```

### Chapter 2: Guessing game

```sh
cargo run --manifest-path rust-learning/Chapter_2/guessing_game/Cargo.toml
```

Enter a whole number from 1 to 100 and follow the hints until you win.

### Chapter 3: Common programming concepts

```sh
cargo run --manifest-path rust-learning/Chapter_3/common_concepts/Cargo.toml
```

The coins exercise is a standalone Rust file without a Cargo manifest:

```sh
mkdir -p rust-learning/Chapter_3/Variables/target
rustc rust-learning/Chapter_3/Variables/src/bin/coins.rs -o rust-learning/Chapter_3/Variables/target/coins
./rust-learning/Chapter_3/Variables/target/coins
```

To run the other variable exercises, replace `coins.rs` and the output name
`coins` with `practice.rs` / `practice` or `types.rs` / `types`.

### Chapter 4: Ownership and borrowing

```sh
mkdir -p rust-learning/Chapter_4/target
rustc rust-learning/Chapter_4/example1.rs -o rust-learning/Chapter_4/target/example1
./rust-learning/Chapter_4/target/example1
```

Use the same pattern for `example2.rs` through `example5.rs`, changing both
the source filename and executable name.

### Exercism practice

```sh
mkdir -p rust-learning/Exercism/target
rustc rust-learning/Exercism/TwoLeap.rs -o rust-learning/Exercism/target/TwoLeap
./rust-learning/Exercism/target/TwoLeap
```

Use the same pattern for the other `.rs` files in `Exercism/`.

## Learning status and notes

- **Chapter 3:** the Cargo example covers the main concepts; the progress notes
  track additional guided practice and pending review. `functions.rs` is currently
  empty and cannot be run as a standalone program.
- **Chapter 4:** ownership and borrowing examples are present. Independent review
  and slice practice remain pending in the progress notes.
- **Chapter 5:** structs practice has started. `struct1.rs` is an unfinished draft
  that does not currently compile.

Detailed learning notes:

- [Chapter 3 progress](rust-learning/Chapter_3/PROGRESS.md)
- [Chapter 3 remaining practice](rust-learning/Chapter_3/FINISH_CHAPTER_3.md)
- [Chapter 4 progress](rust-learning/Chapter_4/PROGRESS.md)
