# Progress

Scores: 0 = can't write it on my own · 1 = can with hints · 2 = can on my own · 3 = can use it in an unfamiliar problem

## Day 1 — Session 1: Language fundamentals

| Drill | Status | Date | File |
|---|---|---|---|
| 1.1 Convert Celsius to Fahrenheit | Done | 2026-11-09 | `src/level_01_fundementals/drill_1.rs` |
| 1.2 Find the largest element in an array | Done | 2026-11-09 | `src/level_01_fundementals/drill_2.rs`, `drill_2_recall.rs` |
| 1.3 Reverse a string without a built-in reverse | Done | 2026-11-09 | `src/level_01_fundementals/drill_3.rs` |
| 1.4 Return multiple values using a tuple | Done | 2026-11-09 | `src/level_01_fundementals/drill_4.rs` |
| 1.5 Match an integer against ranges | Done | 2026-11-09 | `src/level_01_fundementals/drill_5.rs` |
| 1.6 Rewrite a `match` using `if let` | Done | 2026-11-09 | `src/level_01_fundementals/drill_6.rs` |
| 1.7 Shadowing, loop labels and `break` with a value (added) | Done | 2026-11-09 | `src/level_01_fundementals/drill_7.rs` |

Session 1 is complete.

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
| `String` and `&str` | 2 | Borrowed input and returned an owned string in drill 1.3 |
| Characters vs bytes in UTF-8 | 1 | Wrote `.chars()` correctly, but the byte-reversal answer needed correcting |
| `while let` with `pop` | 2 | Used in `vec_reverse` after a hint, with no `if`, indexing or placeholders |
| Tuples (return, destructuring, `.0`) | 2 | All three forms used in drill 1.4 |
| Early returns | 2 | Zero check in `divide` returns early with `return` |
| Integer division with negatives | 2 | Worked out `-7 / 2` and `7 / -2` correctly from the rule |
| Range patterns and `\|` patterns | 2 | Correct in drill 1.5 |
| `match` arm order | 2 | Ran the experiment; moving the range first gives an unreachable-pattern warning |
| `if let` | 2 | Rewrite correct; the reason it fits one case needed refining |
| `let else` | 2 | Correct in drill 1.6 |
| Shadowing | 2 | Separate `let` steps in `parse_and_double` after review |
| Labelled `break` | 2 | Correct in `first_negative`, and now tested with two negatives |
| `break` with a value | 1 | Needed hints, and the reason it only works in `loop` wasn't answered |
| Overflow and `checked_*` methods | 1 | Needed a hint to find and use `checked_mul` |

### Gaps to revisit
- Use known answers from outside the code as expected values in tests. In drills 1.1 and 1.4, the expected values came from the code being tested, so the tests could only agree with it.
- Remove unnecessary special cases: seen in drills 1.2 and 1.3, and not in the final version of 1.7.
- Explain why `break` with a value only works in `loop`: a `for` or `while` can end without reaching a `break`, so there may be no value. Ask again later.
- Write one test per function: drill 1.6 tests three functions in one test.
- Simplify `vec_reverse` in drill 1.3: `output` can be a `String` from the start, without a second `Vec`.
- `?` on `Option` hasn't been practised in a drill yet (Day 2, Session 2).
- Never change correct code just to make a test pass. Fix the test instead.
- Choosing the starting value for a running maximum or minimum: starting at 0 caused a bug with negative numbers.
- Why `&` and `ref` go together in slice patterns: binding `rest @ ..` by value is an unsized-type error (E0277).
- Add a failure message to `assert!` that prints the values (drill 1.1 still lacks one).
- Spelling: rename the folder `level_01_fundementals` to `level_01_fundamentals`.

## Day 1 — Session 2: Structs, enums and data modelling

| Drill | Status | Date | File |
|---|---|---|---|
| 2.1 Three kinds of struct | Done | 2026-11-10 | `src/level_02_structs_enums/drill_1.rs` |
| 2.2 Methods with `self`, `&self` and `&mut self` | Next | | |

### Mastery scores

| Topic | Score | Evidence |
|---|---|---|
| Named, tuple and unit structs | 2 | All three defined correctly; asked what a tuple struct is first |
| Field shorthand | 2 | Used in `new` without hints |
| `Self` inside an `impl` | 2 | Switched to `Self` after one prompt |
| Methods with `&self` returning a new value | 2 | Chose `with_port(&self, …)` and its name |
| Deriving `Debug` and `PartialEq` | 2 | Used in `assert_eq!` on whole structs |
| Struct update syntax (`..base`) and partial moves | 1 | Prediction was 1/3 right; understood after running the experiment; `..*self` explained after a hint |
| Why a field is copied or moved | 1 | Explained twice as "heap allocated" or "stack allocated" instead of `Copy` |

### Gaps to revisit
- Copy vs move depends on whether the type implements `Copy`, not on stack vs heap. Ask again in Session 3 (ownership).
- `..base.clone()` clones every field, then throws away the ones you set yourself. Know the cost compared with cloning only the field you need.
- Test assertions: used a weak `assert_ne!` where the exact value was known, and later expected `10` where the default is `100`. Check expected values against the requirement before running.
- `test_server_config_from_base` still lacks an assertion on `derived.host`, and has a duplicated comment line.

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
