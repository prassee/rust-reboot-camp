# Progress

Scores: 0 = can't write it on my own · 1 = can with hints · 2 = can on my own · 3 = can use it in an unfamiliar problem

## Day 1 — Session 1: Language fundamentals

| Drill | Status | Date | File |
|---|---|---|---|
| 1.1 Convert Celsius to Fahrenheit | Done | 2026-11-09 | `src/level_01_fundementals/drill_1.rs` |
| 1.2 Find the largest element in an array | Done | 2026-11-09 | `src/level_01_fundementals/drill_2.rs`, `drill_2_recall.rs` |
| 1.3 Reverse a string without a built-in reverse | Next | | |
| 1.4 Return multiple values using a tuple | Not started | | |
| 1.5 Match an integer against ranges | Not started | | |
| 1.6 Rewrite a `match` using `if let` | Not started | | |

### Mastery scores

| Topic | Score | Evidence |
|---|---|---|
| Casting with `as` | 2 | Wrote the `i32` to `f64` cast without hints |
| Function body as a final expression (no `return`) | 2 | Done in drills 1.1 and 1.2 |
| Constants (`const`) | 2 | `TOLERANCE` written correctly during reconstruction |
| Comparing floats within a tolerance | 1 | Needed hints, and first rounded the function's result instead of fixing the test |
| Slices as parameters (`&[T]`) | 2 | Chose `&[i32]` without help |
| `Option` for empty input | 2 | Moved from returning 0 to `Option<i32>` |
| Slice patterns (`[first, rest @ ..]`) | 2 | Needed two hints, then reconstruction succeeded |
| Default binding modes in patterns | 2 | Explained the types of `first` and `rest` after one correction |

### Gaps to revisit
- Use known answers from outside the code as expected values in tests. Twice the expected values repeated the function's own formula, so the tests could only agree with it.
- Never change correct code just to make a test pass. Fix the test instead.
- Choosing the starting value for a running maximum or minimum: starting at 0 caused a bug with negative numbers.
- Why `&` and `ref` go together in slice patterns: binding `rest @ ..` by value is an unsized-type error (E0277).
- Add a failure message to `assert!` that prints the values (drill 1.1 still lacks one).
- Spelling: rename the folder `level_01_fundementals` to `level_01_fundamentals`.

## Before the current plan — Exercise 1: Job state model (closed early, code deleted)

| Topic | Score | Evidence |
|---|---|---|
| Struct definition and `new` constructor | 2 | Written correctly without help |
| Enums with data-carrying variants | 2 | `Failed(String)` modelled correctly |
| `match` and destructuring | 2 | Bound the reason in `Failed`, and later dropped `&` once it wasn't needed |
| Owned `String` parameters | 2 | Moved `reason` straight into `Failed` |
| `self`, `&self` and `&mut self` | 1 | Asked whether mutating through `&mut self` was useful, and tried a consuming `complete(mut self)` |
| Implementing `Display` | 1 | Infinite recursion on the first try; `write!` not used yet |
| Designing a `Result`-based API | 1 | Needed several hints to replace silent no-ops with `Result` |
| Testing behaviour, not just return values | 1 | Needed a hint to check state instead of only `is_ok()` |

### Gaps to revisit (planned for Day 1, Sessions 2–3, and Day 2, Session 2)
- Return a reference from a getter instead of building a new `String`.
- Use `write!` inside a `Display` implementation.
- Never use `println!` inside a library type.
- Decide whether repeated transitions (such as starting a job that's already running) should be errors or allowed.
- Write tests for invalid state transitions.
