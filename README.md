# aureate

Rust learning exercises, organized by chapter under `rust-learning/`.

## Repository structure

```text
rust-learning/
├── Chapter_1/                  # Getting started
│   ├── hello_world/            # First program, compiled with rustc
│   └── hello_cargo/            # Hello, world! using Cargo
├── Chapter_2/                  # Guessing game
│   ├── guessing_game/          # Working number-guessing game
│   └── guessing_game_starter/  # Original starter, still prints Hello, world!
└── Chapter_3/                  # Common programming concepts
    ├── common_concepts/        # Fundamentals practice using Cargo
    └── Variables/              # Standalone coins exercise
        └── src/bin/coins.rs
```

`common_concepts` contains practice with variables, mutability, shadowing,
constants, data types, tuples, arrays, functions, expressions, conditions, and
loops. It belongs in Chapter 3 alongside the smaller `Variables` exercise, which
practices updating a mutable coin count using a constant bonus.


### Chapter 1: Hello

```sh
rustc rust-learning/Chapter_1/hello_world/main.rs -o rust-learning/Chapter_1/hello_world/main
./rust-learning/Chapter_1/hello_world/main

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
