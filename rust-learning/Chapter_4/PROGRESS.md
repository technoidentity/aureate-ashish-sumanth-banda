# Chapter 4 learning progress

Last updated: 2026-09-19.

Source: [Brown Rust Book — Understanding Ownership](https://rust-book.cs.brown.edu/ch04-00-understanding-ownership.html).

## Current position

- Current lesson is the requested 4.5 ownership recap. Chapter 4 explanations and summaries now span 4.1 through 4.5; guided ownership and reference exercises are reviewed. The learner requested summaries rather than completing the outstanding independent checks. Teaching coverage is complete after this recap; independent mastery and slice practice remain unverified.
- Reviewed `example1.rs`: `coins` is declared in `main`, `label` is declared in an inner block, and both prints are now in valid positions. The learner confirmed the program works.
- Explained that the inner block can access `coins`, while `label` is unavailable after that block. The owned string is dropped at the end of its scope.
- Memory safety, stack frames, heap storage, pointers, and the local `String` handle were introduced. Independent understanding is still to be checked.
- Reviewed the updated `example2.rs`: statements are inside `main`; `number` is assigned to `another`; `name2 = name.clone()` preserves both string bindings; all four bindings are printed. The learner reports completion and understanding of cloning. This version demonstrates guided integer copying and string cloning; independent move reasoning and mutation of one clone remain for the recap.
- Clarify the learner's summary: local `let` statements and prints in these programs go inside functions, but function/type/constant definitions can be at module level. Rust is case-sensitive: use `let name = String::from("Ashish");`.
- Reviewed corrected `example3.rs`: `pass_on(mut text: String) -> String` appends with `push_str` and returns final `text`; `main` transfers `name` and prints `returned`. The caller's initial `"Ashish "` supplies the separating space. The learner confirmed understanding after an execution trace; independent ownership explanation remains for the recap.
- Explained that `push_str` appends to the existing string and does not print. Execution starts in `main`, creates the string, calls `pass_on`, returns the modified string, then prints it. Function definition order does not determine text order.
- Explained function definitions versus calls, a minimal custom `struct` type, and a typed constant; detailed structs remain Chapter 5 material.
- Current checkpoint: summarise 4.5, connecting runtime storage and cleanup, compile-time permissions, ownership versus garbage collection, and prevention of invalid memory access. The requested 4.3 and 4.4 summaries were delivered. The unanswered 4.2/Box questions and independent slice/error-fixing exercises remain optional follow-up practice; do not block the requested recap or claim these checks were passed.
- Reviewed `example5.rs` and the learner's reported output `15`: `add_bonus(points: &mut i32)` performs `*points += 5`; `main` creates mutable `score = 10`, calls the function with `&mut score`, then prints `score`. This correctly mutates the caller's value without a return value or clone. Function construction has improved: the learner correctly wrote the new helper and call from the supplied requirements. Independent recall remains for the final check.
- Reviewed reordered `example4.rs` and the learner's execution output: `add_words(&mut name)` occurs before `let reader = &name`, followed by prints of both `name` and `reader`. This is a correct alternative repair: finish mutation before creating the shared reference. Both prints read the updated string, so their shared accesses are compatible. The suffix still lacks a leading space, explaining the two `Ashishis learning rust` lines; this is only formatting. This solution does not yet demonstrate ending a shared borrow at its last use before a later mutation.
- Checked items record reviewed guided practice, not independent mastery. Update after reviewing each attempt; do not mark a section complete just because its explanation was given.

## 4.1 What Is Ownership?

- [x] Create an owned string with `String::from` and print it.
- [x] Practice nested scopes and observe an out-of-scope variable error.
- [ ] Explain why memory must remain valid while it is used; distinguish compiler checks from runtime cleanup.
- [ ] Trace stack values, heap allocations, and an owning pointer using `Box::new` and `String`.
- [x] Assign an integer to a second binding and use both in a guided program.
- [x] Create a string with `.clone()` and use both owned strings in a guided program.
- [ ] Independently explain `Copy` versus use-after-move and predict mutation of one cloned string.
- [x] Pass an owned string into a function, modify it, return it, and print its new owner in a guided program.

## 4.2 References and Borrowing

Teaching coverage: the final combined explanation was delivered. Independent review is deferred to 4.5 at the user's requested pace; do not mark independent items complete merely from the explanation.

- [x] Pass `&name` into a function accepting `&String` and use the owned string afterward in a guided program.
- [x] Use `*` to modify a caller's integer through a mutable reference in a guided function exercise.
- [x] Modify a caller-owned string through an `&mut String` function parameter and use the owner afterward in a guided program.
- [ ] Independently distinguish a mutable binding from a mutable reference.
- [x] Repair a conflicting borrow example by completing mutation before creating the shared reference.
- [ ] Explain shared versus exclusive access to the same data and why mutation can invalidate references.
- [ ] Trace Brown's R/W/O permissions on a variable and its referent; explain when a borrow ends.
- [ ] Explain why referenced data must outlive its uses, including returned references and Brown's F (flow) permission. Detailed lifetime syntax belongs to Chapter 10.

## 4.3 Fixing Ownership Errors

Requested summary: match fixes to intended ownership, mutation, and lifetimes; cover collection element access and conservative borrow checking. No new exercise is assigned in this summary turn.

- [ ] Fix a returned reference to local data and choose function inputs according to intended ownership and mutation.
- [ ] Resolve a borrow/mutation conflict by shortening the borrow or retaining only the needed copied information.
- [ ] Access collection elements appropriately with borrowing, copying, cloning, or removal.
- [ ] Understand conservative borrow checking: separate tuple fields, function signatures, and separate array elements using `split_at_mut`.

## 4.4 The Slice Type

Requested summary: borrowed views, range syntax, String versus &String versus &str, UTF-8 byte boundaries, slice-returning functions, protection against stale indices, and general array slices. Guided slice practice is still pending.

- [ ] Create string and array slices; explain ranges, borrowed views, and pointer/length metadata.
- [ ] Distinguish `String`, `&String`, and `&str`, including string literals and flexible `&str` parameters.
- [ ] Return a first-word slice and explain why it protects against mutation while still used, unlike a saved numeric index.
- [ ] Explain byte indexing and valid UTF-8 boundaries for string slices.

## 4.5 Ownership Recap

Summary coverage: ownership and predictable cleanup versus tracing garbage collection; runtime memory versus compile-time permission checks; moves, Copy, cloning, borrowing, dereferencing, slices, and prevention of use-after-free/double-free. A worked example is explanatory, not evidence of independent mastery. No exercise is required in response to the user's summary request.

- [ ] Relate ownership and borrowing to memory safety, predictable cleanup, and garbage collection.
- [ ] Complete one fresh combined exercise and explain its ownership/borrowing behavior without copying a solution.

## Teaching routine and next exercise

- The learner wants to finish efficiently. Group related concepts into short lessons and one focused exercise each; avoid long worksheets or repeated checks after a concept is demonstrated.
- Latest pacing correction: the learner expressed frustration and asked whether we were still in 4.1 after the Box detour. Clearly report that 4.2 is current and distinguish topics explained from independent checks. Stop assigning a new program or waiting for a prediction after every small concept. Finish the remaining 4.2 ideas together, then teach 4.3 and 4.4 in coherent sections; use one focused exercise per section and a combined 4.5 recap. Provide extra scaffolding when a specific obstacle arises without restarting already covered material.
- Adapt to the latest feedback: temporarily scaffold one function at a time. Use the routine job -> input name/type -> body -> call from `main`; have the learner fill small gaps and later reproduce the structure. Do not introduce further borrowing rules before the definition/call connection is understood.
- Borrowing syntax clarification has been delivered and acknowledged: `String` is a type, `name` is an actual variable; parameter types specify the required reference, and argument expressions create/pass that reference. `show` reads through a shared reference; `add_words` changes the caller's string through a mutable reference. Continue the borrow-conflict exercise, but do not treat acknowledgment as independent mastery or mark borrowing rules complete yet.
- Show the current checkpoint and keep this file updated so future sessions resume at the correct point.
- Prefer small illustrative snippets, then learner-written code. Give hints first when debugging. Clearly label intentional compilation failures and explain how to restore a working version.
- Compiler basics practiced: compile the source with `rustc example1.rs`, run the successful output with `./example1`, put executable statements inside a function, and separate consecutive print statements with semicolons. The missing `main` wrapper recurred in `example2.rs`; remind the learner explicitly when snippets belong inside `main`.
- Guided copy/clone exercise (`example2.rs`) is reviewed. Avoid repeating the syntax drill; test independent move reasoning and clone mutation in a later combined check.
- Guided function-ownership exercise (`example3.rs`) is reviewed. Typed parameters, `-> String`, final expressions without semicolons, and binding-specific mutability were explained.
- Borrow-conflict reordering exercise is reviewed. Accept the learner's alternative ordering (mutate, then borrow and read); do not require the previously suggested read-then-mutate sequence. Revisit last-use reasoning in a later compact check.
- Numeric-reference exercise (`example5.rs`) is reviewed. Explain that the successful `*points += 5` changes `score` itself and that method-call syntax follows references automatically for earlier String examples.
- Box/memory catch-up has been explained with `Box::new(10)`, a move to `second`, dereferencing, a memory diagram, and cleanup at scope end. Defer the unanswered move/clone prediction questions to the combined recap; do not block 4.2 on a separate Box exercise. Owning Box versus non-owning reference and Copy as a type property were introduced.
- Explain that `mut text: String` owns the argument, while `text: &mut String` borrows it. The latter can modify the referent without a mutable binding named `text`.
- Chapter 4 teaching coverage finishes with the requested 4.5 summary. Preserve the unchecked independent items for later practice; the learner can proceed to Chapter 5 when requested. Do not equate summary coverage with demonstrated mastery or assign a new test as a prerequisite to moving on.
- Single 4.2 check after the combined lesson (answer in chat; no new file required): for mutable `score = 10`, borrow `reader = &score`, print `reader`, then borrow `writer = &mut score`, add 5 through `*writer`, and print `score`. Ask for output, why the borrow sequence compiles, whether moving the first print after creation of `writer` would compile, and why a helper cannot return a reference to its newly created local String. Review this once, then proceed to 4.3; retain later independent checks for 4.5.
