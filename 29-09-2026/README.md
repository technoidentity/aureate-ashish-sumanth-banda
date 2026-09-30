# Learning Report — 29 September 2026

The day's work covered Rust problem solving, Linux text-processing commands, Git collaboration, and office learning sessions. Rust practice focused on applying concepts from chapters 1–6 through standalone programs, including input validation, structs, methods, enums, borrowing, optional results, and iterative calculations.

## Rust practice

Worked through the implementation and review of Rust programs.
 
| Problem | Implementation and concepts practised | Current example output |
|---|---|---|
| [Order Item](problems/order_item.rs) | Validated positive price and quantity, calculated total cost, and identified bulk orders using an optional constructor and methods. | `Invalid item` for a zero quantity |
| [Validated Student](problems/validated_student.rs) | Accepted scores from 0–100, returned an optional Student, and classified the result through a Pass/Fail enum. | `Ashish`, then `Pass` |
| [Command Descriptions](problems/Command_descriptions.rs) | Matched enum variants containing data and built descriptions using borrowed values, `String`, `push_str`, and numeric conversion. | `add bread`; `Remove item 5`; `list items`; `clear the list` |
| [First Long Name](problems/first_long_name.rs) | Searched a borrowed slice using `chars().count()` and returned an optional reference to the first name with a count greater than five. | `No long name found` for pen, apple, and bag |
| [First Passing Student](problems/first_passing.rs) | Combined structs, an outcome method, enum matching, and slice iteration to return the first passing student. | `Ravi` |
| [Collatz Conjecture](problems/gigaseconds.rs) | Applied even/odd transitions in a while loop, maintained a step counter, and rejected zero input using `Option`. | `2` steps for input 4 |

The Collatz implementation is currently stored in `gigaseconds.rs`. The date-and-time Gigasecond exercise remains pending.

## Technical blockers and resolutions

| Blocker | Resolution |
|---|---|
| Connecting validation, construction, and later operations on a value | Used associated builders returning `Option<Struct>`, extracted valid instances through `match`, and called methods on those instances. |
| Handling different enum types and producing readable output | Matched each variant, accessed its contained value, converted numeric data with `to_string()`, and appended borrowed text with `push_str()`. |
| Searching inputs of varying lengths while preserving ownership | Replaced fixed-position checks with iteration over borrowed slices and returned existing elements through `Option<&T>`. |
| Ending a search before all relevant elements had been examined | Returned immediately when a match was found and placed the no-result outcome after the loop. |
| Maintaining state and counting all Collatz transitions | Updated existing mutable variables, used division for the even transition, and returned the accumulated step count after the stopping condition was reached. |
| Connecting associated functions, borrowed arguments, and optional results in main | Called functions through the appropriate type or function name, borrowed input collections, and matched returned optional values before displaying their contents. |

## Linux command-line session

Attended a **1 hour 30 minute** session during the first part of the day covering:

- **Pipes (`|`):** passing one command's standard output to another command's standard input.
- **`cut`:** selecting fields or columns from text.
- **`grep`:** filtering lines using search patterns.
- **`uniq`:** identifying or removing adjacent duplicate lines.

The session covered combining these commands into text-processing pipelines.

## Git and pull-request workflow

Raised my first pull request after resolving a branch-history blocker.

The two branches did not share a common history. I created a separate branch, merged the work from both existing branches, and raised the pull request from the resulting branch. This provided practical experience with integrating work across branches and preparing changes for review.

## Additional learning sessions

- Attended a **1-hour heart health session** conducted in the office.
- Completed **1 hour of non-technical study**.

## Validation

All six Rust source files compiled successfully with `rustc`, and their current main-function examples were executed. The observed results are recorded in the table above.

These checks cover the supplied local examples. The official Exercism test suites and the planned independent mock assessment were not completed as part of this validation.

## Follow-up

- Add boundary-case checks and repeat selected exercises independently.
- Align the Collatz filename and the Order Item quantity field name with their purpose.
- Normalize the removal-command description to the required lowercase wording.
- Revisit Gigasecond after an introduction to its date-and-time library.

