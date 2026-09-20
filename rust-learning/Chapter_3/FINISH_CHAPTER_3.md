# Finish Chapter 3: three practical exercises

Work beside `types.rs`. Write the programs yourself; these notes provide syntax examples and requirements, not complete solutions. Submit one finished exercise at a time, or continue through all three and submit them together.

Compile with `rustc filename.rs`. If compilation succeeds, run `./filename` (without `.rs`).

## Exercise 1 — finish data types: `profile.rs`

### Quick reference

- **Type inference:** Rust works out types from values and how they are used. When nothing else constrains them, integer literals default to `i32` and floating-point literals default to `f64`.
- **Type annotation:** `let price: f32 = 12.5;` explicitly chooses `f32`. `f32` uses 32 bits; `f64` uses 64 and provides more precision. Floating-point values approximate many decimal fractions.
- **Characters:** `'R'` is a `char`; `"R"` is a string literal of type `&str`. A `char` represents one Unicode scalar value, such as `'R'` or `'🦀'`.
- **Tuples:** group a fixed number of values, possibly of different types. For `let item = ("pen", 3, true);`, use `item.0` for the first field. `let (name, count, available) = item;` creates separate bindings for the fields; this is called destructuring.
- **Arrays:** group a fixed number of values of the same type. `let marks: [i32; 3] = [50, 60, 70];` contains three integers. `marks[0]` is the first element; `marks.len()` is the number of elements. `let zeros = [0; 3];` creates three zeros.
- Tuple fields and array indices start at **0**. An array of length 3 has valid indices 0, 1, and 2. A known invalid index can be rejected at compilation; an invalid index checked during execution causes a panic.

### Build a player profile

Inside `main`:

1. Declare a name containing `"Ashish"`, a character badge `'R'`, an explicitly typed `f32` accuracy of `92.5`, and an unannotated decimal speed of `2.5`.
2. Create a tuple named `player` containing the name, badge, and accuracy.
3. Print the badge using tuple field access, then destructure `player` and print all three resulting values. Print speed too.
4. Create a `scores` array containing `40`, `65`, and `80`. Print its first element, last element, and length.
5. Create an array of three repeated zeros and print it. To print an entire array, use debug formatting: `println!("{zeros:?}");`.
6. Declare and print `1_000`, `0xff`, `0o17`, `0b1010`, and `b'A'`. Predict their decimal values before running. These are a decimal with a readability separator, hexadecimal, octal, binary, and a byte literal. A byte literal has type `u8`.

### Check your understanding

- Which type does the unconstrained `speed` variable get?
- Why is `scores[3]` invalid? Try accessing it after your normal program works, observe the diagnostic, then restore a valid index.
- Why can a tuple mix text and numbers while an array cannot mix element types?

Some operations need extra type information. In a separate experiment, try:

```rust
let parsed: u32 = "42".parse().expect("Expected a number");
println!("{parsed}");
```

Remove only `: u32`, compile, read the diagnostic, then restore it. The target type tells `parse` what to produce.

**Overflow recap:** your `u8` addition panicked with overflow checks enabled. Default debug builds enable these checks; default release builds normally disable them and overflowing integer arithmetic wraps. Build settings can override that behavior. Do not rely on overflow to produce an ordinary arithmetic result. Rust also provides explicit `checked_*`, `wrapping_*`, `overflowing_*`, and `saturating_*` methods when those behaviors are needed.

## Exercise 2 — functions and comments: `functions.rs`

### Quick reference

A function packages reusable work. Parameters are named inputs with explicit types. Arguments are the actual values supplied when calling it.

```rust
fn double(value: i32) -> i32 {
    value * 2
}
```

Call this function with `double(6)`. The `-> i32` declares its return type.

- `let doubled = double(6);` is a statement.
- `double(6)` and `value * 2` are expressions: they produce values.
- A block can produce a value from its final expression without a semicolon. In this example, that is how the function returns its result.
- A semicolon after that final expression discards its value; the block then yields `()`, called **unit**, rather than the required integer.
- `return value * 2;` is an explicit return and exits the function immediately.
- A function without a specified return type returns `()`. This is also the empty tuple.
- `//` starts a comment that continues to the end of the line. `/* ... */` can contain a block comment. Use comments to explain a reason that the code alone does not make clear.

### Build a coin calculator

1. Define `share_coins` with two parameters, `coins` and `players`, both `u32`. Return a `u32` using integer division. Use a final expression without a semicolon.
2. Call it from `main` with `(17, 5)` and `(20, 5)`, storing and printing both results. Use positive player counts in this exercise.
3. Define `print_badge` with a `char` parameter and no explicit return type. Have it print the badge, then call it from `main`.
4. Add a `//` comment explaining why integer division suits whole coins. Add one short block comment as syntax practice.
5. Create a variable whose value comes from a block: declare a local base of `4` inside the block, then make the block yield twice that value. Print the result.

### Experiments

- Add a semicolon to the final division expression in `share_coins`. Read the compiler error, then repair it.
- Rewrite its return using `return`, preserving the result.
- Store the result of a call to `print_badge` and print that result using `{:?}`. Explain why the badge function's return value is `()`.

## Exercise 3 — control flow: `rounds.rs`

### Quick reference

Conditions must have type `bool`. Rust does not automatically treat an integer as true or false.

```rust
let status = if ready { "go" } else { "wait" };
```

This assumes a boolean named `ready`. An `if` can produce a value; its branches must produce compatible types. Use `else if` for additional conditions.

| Construct | Job |
| --- | --- |
| `loop { ... }` | Repeat until a `break` exits it |
| `break value;` | Exit a `loop` and supply its result |
| `while condition { ... }` | Repeat while a boolean condition is true |
| `for item in collection { ... }` | Visit each item |
| `continue;` | Skip the remainder of the current iteration |
| `1..4` | Range including 1, 2, and 3 |
| `1..=4` | Range including 1, 2, 3, and 4 |
| `(1..=3).rev()` | Visit 3, 2, then 1 |

A plain `break` exits the innermost loop. A label allows a break to target an outer loop:

```rust
'outer: loop {
    loop {
        break 'outer;
    }
}
```

### Build a game-round report

1. Declare `PASS_MARK` as a constant with value `50` and create an array of scores `[40, 65, 80]`.
2. Write a function that adds a supplied bonus to a score and returns the new score. Call it for each score using a `for` loop and a bonus of `5`.
3. Within that loop, create a result label using an `if` expression: `"excellent"` for a boosted score of at least 80, `"pass"` for at least `PASS_MARK`, otherwise `"retry"`. Print each boosted score and label.
4. Add a separate `for` loop over the original scores that uses `continue` to skip scores below `PASS_MARK`, then prints the remaining scores.
5. Use a mutable countdown and `while` to print `3`, `2`, `1`.
6. Use `loop` to count attempts from zero. On the third attempt, exit using `break` with a value equal to ten times the attempt count. Store the loop's result and print it.
7. Print `1..4` and `1..=4` using separate `for` loops and compare the results. Then print a countdown using `.rev()`.
8. Add nested loops for rows `1..=2` and columns `1..=3`. Print visited row/column pairs. When you reach row 2, column 2, exit both loops using a label **before** printing that pair. To require both comparisons, combine them using `&&`.
9. Add comments where they explain your decisions.

### Independent final check

Change the score array and bonus. Predict all result labels before running. Explain why the inner plain `break` and a labeled outer `break` would stop at different places. Explain which bindings need `mut` and which do not.

## Completion check

The chapter is complete when the three exercises have been reviewed, intentional errors have been explained, and the independent check works. Reading these notes alone does not mark exercises complete. Progress is tracked in `PROGRESS.md`.

References: [Data types](https://doc.rust-lang.org/book/ch03-02-data-types.html), [Functions](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html), [Comments](https://doc.rust-lang.org/book/ch03-04-comments.html), [Control flow](https://doc.rust-lang.org/book/ch03-05-control-flow.html).
