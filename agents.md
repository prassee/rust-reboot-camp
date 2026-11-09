# Rust Mastery --- Two-Day Intensive Reboot

## Purpose and learning principles

Rebuild independent Rust coding fluency, from core syntax to advanced
language and systems concepts. The goal is to write Rust without
depending on AI coding agents, understand compiler diagnostics,
recognise idiomatic patterns, and identify areas requiring deeper
practice.

This is a two-day intensive revision and diagnostic sprint, not a claim
that every advanced concept can be mastered in 16 hours.

-   Concept-first, exercise-driven learning; do not make
    application-building the primary activity.
-   Practise one concept at a time through multiple small exercises.
-   Use integration exercises only to validate combinations of concepts.
-   Prefer writing code from memory over passively reading examples.
-   Treat compiler errors as learning opportunities: predict, compile,
    inspect, explain and fix.
-   Do not use an AI coding agent to generate a solution before
    attempting the exercise.

## Practice distribution

-   **50% Syntax drills (3--7 minutes):** short, isolated questions that
    build syntax recall.
-   **35% Concept challenges (10--20 minutes):** deeper exercises
    focused on one concept.
-   **15% Integration challenges (20--40 minutes):** combine previously
    practised concepts.

Examples: - Syntax: write a struct declaration; implement a method;
destructure an enum; write a generic function. - Concept: fix ownership
errors; implement a lifetime-aware function; create a trait with
associated types; build a custom iterator. - Integration: model data
using structs and enums; abstract behaviour with traits and generics;
process collections using iterators and error handling.

## Mastery measurement

    Score Meaning                              Next step
  ------- ------------------------------------ ----------------------
        0 Cannot write independently           Review and retry
        1 Can write with hints                 More syntax drills
        2 Can write independently              Move to next concept
        3 Can apply in an unfamiliar problem   Consider mastered

A topic should reach level 2 before moving forward. Level 3 can be
validated later through integration exercises. Record topics scoring 0
or 1 for follow-up practice.

------------------------------------------------------------------------

# DAY 1 --- Language Foundations, Ownership and Lifetimes

**Suggested duration: 7--8 hours**

## Session 1 --- Language fundamentals (75 minutes)

### Concepts

1.  Variables: `let`, `mut`, shadowing and constants.
2.  Types: integers, floating-point numbers, booleans, characters, type
    inference, explicit annotations and casting.
3.  Expressions: expressions versus statements, block expressions and
    return values.
4.  Control flow: `if`, `else`, `loop`, `while`, `for`, loop labels and
    `break` values.
5.  Functions: parameters, return types and early returns.
6.  Compound types: tuples, arrays and slices.
7.  Strings: `String`, `&str`, conversions and UTF-8 basics.
8.  Pattern matching: `match`, `_`, ranges, destructuring, `if let` and
    `let else`.

### Syntax drills

-   Convert Celsius to Fahrenheit.
-   Find the largest element in an array.
-   Reverse a string without a built-in reverse operation.
-   Write a function returning multiple values using a tuple.
-   Match an integer against ranges.
-   Rewrite a `match` expression using `if let` where appropriate.

## Session 2 --- Structs, enums and data modelling (75 minutes)

### Concepts

1.  Structs: named, tuple and unit structs.
2.  Instantiation: struct literals, field shorthand and update syntax.
3.  Methods: `self`, `&self` and `&mut self`.
4.  Associated functions: constructors and `Self`.
5.  Enums: variants and data-carrying variants.
6.  Pattern matching: destructuring structs and enums.
7.  Derive attributes: `Debug`, `Clone`, `Copy`, `PartialEq`, `Eq` and
    `Default`.
8.  Formatting: `Display`, `Debug`, `format!`, `print!` and `println!`.

### Syntax drills

-   Define named, tuple and unit structs.
-   Instantiate structs using field shorthand and struct update syntax.
-   Add methods that read and modify fields.
-   Create an associated constructor using `Self`.
-   Create enums with variants carrying different data types.
-   Destructure nested structs and enums.
-   Implement `Display` for a custom type.
-   Compare instances using `PartialEq`.
-   Practise the different derive attributes.

## Session 3 --- Ownership and memory model (120 minutes)

### Concepts

1.  Ownership rules, scope and moves.
2.  Copy semantics: `Copy` versus `Clone`.
3.  Borrowing and references.
4.  Immutable and mutable references.
5.  Aliasing and mutability restrictions.
6.  Array slices and string slices.
7.  Passing ownership versus borrowing in functions.
8.  Returning owned values and references.
9.  Stack, heap, drop and scope.

### Concept challenges

-   Predict whether a variable remains usable after a function call.
-   Fix move errors in small snippets.
-   Rewrite functions to borrow instead of taking ownership.
-   Modify a vector through a mutable reference.
-   Return a slice from a function and understand why some versions
    fail.
-   Identify invalid combinations of mutable and immutable borrows.
-   Explain when a value is copied, moved or cloned.
-   Identify where ownership is transferred and when a value is dropped.

## Session 4 --- Lifetimes (90 minutes)

### Concepts

1.  Lifetime fundamentals and reference validity.
2.  Lifetime annotations: `'a`, `'b` and lifetime relationships.
3.  Lifetimes on function parameters and return values.
4.  Structs containing references.
5.  Multiple independent and related lifetimes.
6.  The three lifetime elision rules.
7.  `'static`, including string literals.
8.  Lifetime bounds such as `T: 'a`.
9.  Higher-ranked trait bounds: `for<'a>`.

### Concept challenges

-   Identify which references outlive their data.
-   Fix a function returning a reference.
-   Implement a struct holding two references.
-   Explain why a lifetime annotation is required.
-   Identify when the compiler infers lifetimes and when it cannot.
-   Write a generic function with a lifetime bound.
-   Distinguish `'static` from an ordinary inferred lifetime.

## Session 5 --- Day 1 consolidation (30--45 minutes)

### Integration challenge

Create a small collection of independent data types using structs and
enums, methods and associated functions, pattern matching, `Debug` and
`Display`, ownership and borrowing, and at least one lifetime-aware
function or struct.

Do not turn this into an application. The goal is to combine the day's
concepts in a small, self-contained exercise.

### Day 1 checkpoint

Assign a mastery score to every topic. Record topics scoring 0 or 1 for
follow-up practice.

------------------------------------------------------------------------

# DAY 2 --- Abstraction, Advanced Language Features and Systems Programming

**Suggested duration: 7--8 hours**

## Session 1 --- Traits, generics and abstraction (105 minutes)

### Concepts

1.  Traits and trait definitions.
2.  Trait implementations.
3.  Required methods and default methods.
4.  Generic functions, structs and enums.
5.  Trait bounds and `where` clauses.
6.  Associated types versus generic parameters.
7.  Trait objects using `dyn Trait`.
8.  Static versus dynamic dispatch.
9.  `From`, `Into`, `TryFrom` and `TryInto`.
10. Supertraits and blanket implementations.
11. Higher-ranked trait bounds.

### Syntax drills and challenges

-   Write a generic function that finds the maximum value.
-   Define a trait and implement it for multiple types.
-   Add a default method to a trait.
-   Write a function accepting any type implementing a trait.
-   Compare generic dispatch with trait-object dispatch.
-   Implement a custom conversion using `From`.
-   Compare associated types with generic trait parameters.
-   Implement a blanket trait implementation.
-   Practise `where` clauses.

## Session 2 --- Collections, iterators and error handling (105 minutes)

### Concepts

1.  Collections: `Vec`, `HashMap`, `HashSet` and `VecDeque`.
2.  Strings: ownership, slicing, formatting and conversions.
3.  `Option`: `Some`, `None`, `map`, `and_then` and `unwrap_or`.
4.  `Result`: `Ok`, `Err` and error propagation using `?`.
5.  Errors: custom error types and error conversion.
6.  Closures: capture by reference, mutable reference and `move`.
7.  Iterators: `iter`, `iter_mut` and `into_iter`.
8.  Iterator adapters: `map`, `filter`, `flat_map`, `enumerate` and
    `zip`.
9.  Iterator consumers: `fold`, `reduce`, `collect`, `find`, `any` and
    `all`.

### Syntax drills and challenges

-   Group values by a key using `HashMap`.
-   Transform and filter a vector using iterators.
-   Implement a function returning `Result`.
-   Convert between `Option` and `Result`.
-   Write a closure that captures and modifies an external variable.
-   Compare `iter`, `iter_mut` and `into_iter`.
-   Implement a custom iterator.
-   Use `fold` to calculate an aggregate.
-   Chain multiple iterator adapters.

## Session 3 --- Modules, Cargo and advanced language features (105 minutes)

### Concepts

1.  Modules: `mod`, `pub`, `pub(crate)`, `super` and `self`.
2.  File organisation: module declarations and separate module files.
3.  Crates: binary crates and library crates.
4.  Cargo: dependencies, features and workspaces.
5.  Smart pointers: `Box`, `Rc` and `Arc`.
6.  Interior mutability: `Cell`, `RefCell` and `Mutex`.
7.  `Drop` and `Deref`: destruction and dereferencing.
8.  Macros: declarative `macro_rules!`.
9.  Unsafe Rust: raw pointers, unsafe blocks and invariants.
10. Advanced types: const generics, type aliases and newtypes.
11. Advanced traits: operator overloading, `Deref` and `Drop`.

### Syntax drills and challenges

-   Split a module into separate files.
-   Restrict visibility across modules.
-   Build a linked structure using `Box`.
-   Compare `Rc<RefCell<T>>` with `Arc<Mutex<T>>`.
-   Write a small declarative macro.
-   Implement a custom type supporting `+` using `Add`.
-   Implement `Drop` for a custom type.
-   Practise const generics.
-   Explain the safety requirements of a small unsafe block.

## Session 4 --- Concurrency and systems programming (105 minutes)

### Concepts

1.  Threads: `std::thread`, spawning and joining.
2.  Communication: channels and message passing.
3.  Synchronisation: Mutexes, RwLocks and atomics.
4.  Thread safety: `Send`, `Sync` and shared ownership.
5.  Async foundations: futures, async functions and `.await`.
6.  Pinning: `Pin`, `Unpin` and why pinning exists.
7.  Tokio: tasks, spawning, channels and runtime.
8.  Async design: cancellation, backpressure and `select`.
9.  Performance: allocation, ownership costs and profiling.
10. Unsafe and concurrency: data races, memory ordering and invariants.

### Syntax drills and challenges

-   Spawn multiple threads and collect results.
-   Share state safely across threads.
-   Implement producer-consumer communication.
-   Compare thread-based and async approaches.
-   Build a bounded Tokio channel exercise.
-   Identify why a future may not be `Send`.
-   Explain the purpose of pinning.
-   Identify potential data races and synchronisation requirements.

## Session 5 --- Day 2 consolidation (30--45 minutes)

### Integration challenge

Write a small, self-contained exercise combining structs and enums,
traits and generics, collections and iterators, `Option` and `Result`,
modules, and ownership and borrowing.

If time permits, extend it with thread or async communication. Keep it
small: it is a validation tool, not the main learning activity.

### Day 2 checkpoint

Review all eight levels and assign mastery scores. Record a continuation
list for every topic below level 2.

------------------------------------------------------------------------

# Practice protocol

For every topic:

1.  Present one exercise at a time.
2.  Do not reveal the solution before the learner attempts it.
3.  Provide graduated hints when needed.
4.  Review compilation, correctness, ownership, lifetimes and idiomatic
    usage.
5.  Ask for a reconstruction from memory when appropriate.
6.  Record mastery scores and track weak areas.
7.  Include compiler-prediction, error-fixing and implementation-choice
    exercises.
8.  Avoid app-first learning. Use projects only for occasional
    integration and validation.

If an exercise remains unsolved after a reasonable attempt and two
hints, record it as a gap and move forward. Revisit it during follow-up
practice rather than allowing one exercise to consume the schedule.

## Review checklist

For each submitted solution, assess: - Does it compile? - Is the
behaviour correct? - Is ownership handled intentionally? - Are borrowing
and lifetimes valid? - Is the API design sensible? - Is the
implementation idiomatic? - Are unnecessary clones or allocations
present? - Can the learner explain the compiler diagnostics? - Can the
learner reproduce the solution without looking?

------------------------------------------------------------------------

# VS Code practice environment

Recommended repository structure:

``` text
rust-mastery/
├── Cargo.toml
├── agents.md
├── src/
│   ├── level_01_fundamentals/
│   ├── level_02_structs_enums/
│   ├── level_03_ownership/
│   ├── level_04_lifetimes/
│   ├── level_05_traits_generics/
│   ├── level_06_collections_errors/
│   ├── level_07_modules_advanced/
│   └── level_08_concurrency_async/
└── exercises/
    ├── syntax_drills/
    ├── concept_challenges/
    └── integration_challenges/
```

Keep exercises as small independent files or test cases wherever
practical. Each should compile and run independently where appropriate.

### Tooling

-   `cargo check` --- quick compiler feedback.
-   `cargo test` --- validate behaviour.
-   `cargo clippy` --- idiomatic Rust feedback.
-   `rustfmt` --- formatting.
-   `rustc` diagnostics --- understand compiler errors.

### Before responding

Before responding about submitted code, the agent must run these
commands itself and base its review on the actual output:

``` bash
cargo clippy --all-targets
cargo test -- --no-capture --test-threads=1
```

Do not ask the learner to run or paste this output.

------------------------------------------------------------------------

# After the two days

The two-day sprint establishes a baseline. Continue with focused
practice based on mastery scores.

Suggested follow-up priority: 1. Ownership and borrowing. 2. Lifetimes.
3. Traits and generics. 4. Iterators and error handling. 5. Modules and
Cargo. 6. Smart pointers and interior mutability. 7. Macros and unsafe
Rust. 8. Concurrency and async/Tokio.

The end goal is independent Rust fluency: being able to start from an
empty file, write idiomatic Rust, reason about compiler diagnostics and
make sound design decisions without relying on a coding agent.
