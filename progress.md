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
| 2.2 Methods with `self`, `&self` and `&mut self` | Done | 2026-11-10 | `src/level_02_structs_enums/drill_2.rs` |
| 2.3 Enums with data-carrying variants | Done | 2026-11-10 | `src/level_02_structs_enums/drill_3.rs` |
| 2.4 Implementing `Display` | Done | 2026-11-10 | `src/level_02_structs_enums/drill_4.rs` |
| 2.5 Destructuring nested structs and enums | Done | 2026-11-10 | `src/level_02_structs_enums/drill_5.rs` |
| 2.6 Derive attributes: `Clone`, `Copy`, `Eq`, `Default` | Done | 2026-11-11 | `src/level_02_structs_enums/drill_6.rs` |

Session 2 is complete.

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
| Choosing `self`, `&self` or `&mut self` (drill 2.2) | 2 | All six receivers predicted and written correctly; up from 1 in Exercise 1 |
| Getter returning a reference (drill 2.2) | 2 | `owner()` returns `&str` without allocating; closes an Exercise 1 gap |
| Fields are private by default (drill 2.2) | 1 | Thought `pub` on the struct made fields public |
| `Result` from a method (drill 2.2) | 2 | `withdraw` returns `Result` unprompted and leaves the balance unchanged on error |
| E0382 vs E0507 (drill 2.2) | 1 | Predicted E0507 for use after move; then explained the difference correctly |
| Enum variants: unit, tuple and struct-like (drill 2.3) | 2 | All five variants defined correctly without hints |
| Destructuring enums in `match` (drill 2.3) | 2 | Every variant destructured; `&String` binding predicted correctly |
| `\|` patterns and `..` to ignore data (drill 2.3) | 1 | Combined arms unprompted, but needed two prompts to use `..` |
| Characters vs bytes in UTF-8 (drill 2.3) | 1 | Used `.len()` again; fixed after a second hint |
| Implementing `Display` with `write!` (drill 2.4) | 2 | Two corrections during the drill (`write_fmt`, then a local `String` instead of `f`); reconstruction from memory correct apart from writing `self` for `&self` |
| Reusing another type's `Display` via `{}` (drill 2.4) | 2 | Removed `.to_string()` after the clippy warning |
| Blanket impl behind `to_string` (drill 2.4) | 1 | Knew it depends on `Display`, not that it comes from a blanket `ToString` impl |
| Nested destructuring with literals (drill 2.5) | 2 | `matches!` with nested `0` literals and `\|`, unprompted |
| Destructuring in parameters (drill 2.5) | 2 | `mirror` destructures `Point` in the parameter list |
| Match guards (drill 2.5) | 1 | First time seen; after two hints, rewrote `vertical_x` with a guard |
| `Eq` vs `PartialEq`, and floats (drill 2.6) | 2 | Predicted that `Celsius` can't be `Eq`; reason (`NaN`) added in review |
| `Default` values for fields (drill 2.6) | 2 | Predicted all three defaults correctly |
| `Copy` requires every field to be `Copy` (drill 2.6) | 2 | Ruled out `Copy` for `Settings` because of `String`, using `Copy` as the reason |
| Choosing derives from field types (drill 2.6) | 1 | Needed the trait table for primitives; now knows to check "Trait Implementations" in the docs |
| Refutable vs irrefutable patterns (drill 2.5) | 1 | Explained after correction; E0005 not tried, and `mirror` was taken as the variant-pattern case |

### Gaps to revisit
- Copy vs move depends on whether the type implements `Copy`, not on stack vs heap. Ask again in Session 3 (ownership).
- `..base.clone()` clones every field, then throws away the ones you set yourself. Know the cost compared with cloning only the field you need.
- Test assertions: used a weak `assert_ne!` where the exact value was known, and later expected `10` where the default is `100`. Check expected values against the requirement before running.
- `test_server_config_from_base` still lacks an assertion on `derived.host`, and has a duplicated comment line.
- Boundary values: `withdraw` first rejected the exact balance (`>=` instead of `>`), and the test expected the wrong result. Test the exact boundary from the requirement.
- `deposit` can overflow `u64`; `checked_add` is the fix (identified, not implemented).
- Match guards (`pattern if condition =>`): written after hints in drill 2.5. Use one in a later drill without hints to reach 2.
- E0005: put an enum-variant pattern in a function parameter and read the error. Not done yet.
- Read compiler warnings as clues: in drill 2.4, "unused variable: `f`" showed the output was going nowhere.
- Carry earlier lessons forward: in drill 2.3, `.len()` on text (drill 1.3) and a missing non-ASCII test came back. Before submitting, check the gap list for anything that applies.

## Day 1 — Session 3: Ownership and memory model

| Challenge | Status | Date | File |
|---|---|---|---|
| 3.1 Is it still usable? (prediction) | Done | 2026-11-13 | `src/level_03_ownership/drill_1.rs` |
| 3.2 Fix move errors | Next | | |

### Mastery scores

| Topic | Score | Evidence |
|---|---|---|
| Predicting usability after a call | 2 | 7 of 8 cases predicted correctly, with reasons |
| Move vs `Copy` vs `.clone()` | 2 | Cases A–E all correct |
| Partial move vs whole move | 2 | Got the subtle pair: `t.0` usable after a partial move, `x.0` not after a whole move |
| Borrows end at last use (E0505) | 1 | Needed both orderings run before the rule was clear; first wrote `let _ = &s`, which creates no binding |

### Gaps to revisit
- The compiler blames the **move**, not the later use: E0505 points at the move line and notes "borrow later used here".
- Stack vs heap again: a `(i32, String)` tuple lives on the stack, and only the `String`'s buffer is on the heap.

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
