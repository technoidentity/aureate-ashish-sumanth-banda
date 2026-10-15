# Chapter 3 learning progress

## Current position

- Guided exercises finished for 1 of 5 sections. Core data-type exercises have been practiced; the unchecked items in 3.2 remain for final review.
- Current lesson: 3.3 functions, with 3.4 comments included in the same exercise. Progress is recorded by reviewed exercises rather than a precise percentage.
- A checked item means it has been practiced in our lessons. Independent recall is checked separately.
- Chapter outline: https://doc.rust-lang.org/book/ch03-00-common-programming-concepts.html
- Remaining topics are outlined in `FINISH_CHAPTER_3.md` as optional reference. The learner prefers explanations followed by their own code, rather than a worksheet or large syntax samples to copy.

## 3.1 Variables and mutability — guided practice complete

- [x] Bindings and default immutability
- [x] Reassignment with `mut`, including text and numbers
- [x] Basic constant declaration and use, with an explicit type
- [x] Shadowing: a new binding with the same name
- [x] Inner scope shadowing versus updating an outer variable
- [x] Changing types through shadowing
- [x] Each new binding has its own mutability
- [ ] Solve a fresh combined exercise without solution code

Practice files: `Variables/src/bin/coins.rs`, `Variables/src/bin/practice.rs`.

## 3.2 Data types — in progress

- [x] Basic integers versus floating-point numbers
- [x] Addition, multiplication, integer and floating-point division
- [x] Remainder with `%`
- [x] Boolean values from `==`, and printing the result
- [x] Explicit type annotation with `: u8`
- [x] The `u8` range and rejection of an out-of-range literal
- [ ] Type inference
- [x] Signed and wider integer types: choose `u16` for `300` and `i8` for `-10`
- [x] Arithmetic overflow: observed a runtime panic when adding `1` to `255` in a `u8`
- [x] Resolve that overflow by choosing `u16` for the counter
- [ ] Integer literal forms
- [ ] `f32` and `f64`
- [ ] Characters (`char`) versus string literals
- [x] Create a tuple and read a field with `.1`, then print the extracted value
- [x] Destructure a tuple and print the resulting bindings
- [x] Create an array, read valid indices, and calculate the total of its elements
- [ ] Array length and bounds
- [ ] Independent data types exercise

Practice file: `Variables/src/bin/types.rs`.

## 3.3 Functions — current lesson

- [ ] Define and call functions
- [ ] Parameters and argument types
- [ ] Statements versus expressions
- [ ] Return values and the effect of a trailing semicolon

Practice file: `functions.rs` (currently empty; the learner writes the implementation).

## 3.4 Comments — included in the functions exercise

- [ ] Write useful line comments with `//`

## 3.5 Control flow — upcoming

- [ ] `if`, `else`, and `else if`
- [ ] `if` used to produce a value
- [ ] `loop`, `break`, and returning a value from a loop
- [ ] Loop labels
- [ ] `while`
- [ ] `for` and ranges
- [ ] Independent chapter exercise

## Extra practical skills covered

- Compile with `rustc filename.rs` and run with `./filename`.
- Distinguish source files from executable programs.
- Distinguish warnings from errors.
- Distinguish a compilation error from a runtime panic.
- Address unnecessary parentheses and unused variable warnings.

## Teaching routine

- Show the current section and checkpoint with each new exercise.
- Explain each new concept using the learner's current program, then assign a focused task they write themselves. Group related ideas when this helps maintain pace.
- Use only small illustrative syntax examples; do not provide a complete solution unless requested. Copied examples alone do not establish understanding.
- Give hints before solution code when the learner is debugging.
- Update this checklist after reviewed attempts.
- Next checkpoint: write a coin-sharing function with two typed parameters and a returned value, call it with different arguments, use a block expression, add a helpful comment, and investigate a trailing-semicolon error. Then move to control flow and the remaining data-type review.
