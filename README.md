# Learn-Rust-for-Hacking
Having studied computer science, I think this is time to learn one language for good. Although I have a lot of experience using Java, I want to pivot to the security side and [Rust](https://www.rust-lang.org/) seems to be the perfect choice (for me). There are a lot of languages but since I want to start from the very beginning, why not choose a language which is fast, efficient and memory-safe.

I have no idea what to expect from Rust but will document the journey.

My main sources for help, reference and documentation will be [Rust by Example](https://doc.rust-lang.org/rust-by-example/) and [Black Hat Rust](https://kerkour.com/black-hat-rust).

[Official Rust Docs](https://doc.rust-lang.org/std/)

## Day #1
- Installing Rust - [rustup](https://rustup.rs/)
- [Hello World](https://doc.rust-lang.org/rust-by-example/hello.html)
- [Formatted Print](https://doc.rust-lang.org/rust-by-example/hello/print.html)

## Day #2
- [fmt::Display](https://doc.rust-lang.org/rust-by-example/hello/print/print_display/testcase_list.html)

## Day #3
- [fmt::Debug](https://doc.rust-lang.org/rust-by-example/hello/print/print_debug.html)

## Day #4
- Implementing `fmt::Display` for the first time

## Day #5
- Getting more comfortable with `traits` [1.2.2 - Display](https://doc.rust-lang.org/rust-by-example/hello/print/print_display.html)
- Finishing with some cool `formatting` tricks - [1.2.3 Formatting](https://doc.rust-lang.org/rust-by-example/hello/print/fmt.html)

## Day #6
- Playing with primitives and writing functions - `primitives` [2 - Primitives](https://doc.rust-lang.org/rust-by-example/primitives.html)
- [2.1 Literals and Operators](https://doc.rust-lang.org/rust-by-example/primitives/literals.html)
- [2.2 Tuples](https://doc.rust-lang.org/rust-by-example/primitives/tuples.html)

## Day #7
- Slices are similar to array but... - [2.3 - Arrays and Slices](https://doc.rust-lang.org/rust-by-example/primitives/array.html)

## Day #8
- Custom types - Starting with structures - [3.1 - Structures](https://doc.rust-lang.org/rust-by-example/custom_types/structs.html)

## Day #9
- Completed the activities in `3.1 - Structures`

## Day #10
- Learnt about [3.2 Enums](https://doc.rust-lang.org/rust-by-example/custom_types/enum.html)

## Day #11
- Did some coding using ```use``` for enums. [3.2.1 use](https://doc.rust-lang.org/stable/rust-by-example/custom_types/enum/enum_use.html)

## Day #12
- About [3.3 Constants](https://doc.rust-lang.org/rust-by-example/custom_types/constants.html)

## Day #13
- Coming back to [3.2.2. C-like enums](https://doc.rust-lang.org/rust-by-example/custom_types/enum/c_like.html)

## Day #14
- Finished the [1. Getting Started](https://doc.rust-lang.org/book/ch01-00-getting-started.html) section.
- Useful tips about `cargo` here - [1.3 Hello,Cargo!](https://doc.rust-lang.org/book/ch01-03-hello-cargo.html)

<details>
  
<summary>Takeaway tips</summary>

- We can create a project using `cargo new`.
- We can build a project using `cargo build`.
- We can build and run a project in one step using `cargo run`.
- We can build a project without producing a binary to check for errors using `cargo check`.
- Instead of saving the result of the build in the same directory as our code, Cargo stores it in the `target/debug` directory.
</details>

## Day #15

- Started the module [3. Common Programming Concepts](https://doc.rust-lang.org/book/ch03-00-common-programming-concepts.html) section.
- Studying [3.1 Variables and Mutability](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html)

<details>
  
<summary>Takeaway tips</summary>

- Variable is immutable by default.
- However we can use `mut` to make a variable mutable.
- Constants aren’t just immutable by default—they’re always immutable.
- Constants are valid for the entire time a program runs, within the scope in which they were declared. 
</details>

## Day 16

- Learnt about `shadowing`.
- Completed the [3.1 Variables and Mutability](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html) section.

<details>

<summary>Takeaway tips</summary>

- Difference between `mut` and `shadowing` is that because we’re effectively creating a new variable when we use the `let` keyword again.
- First variable is shadowed by the second, which means that the second variable is what the compiler will see when you use the name of the variable.

</details>

## Day 17

- Learning about `Data types`.
- Currently on [3.2. Data Types](https://doc.rust-lang.org/book/ch03-02-data-types.html) section.

<details>

<summary>Takeaway tips</summary>

- Rust is a *statically* typed language, which means that it must know the types of all variables at compile time.
- A *scalar* type represents a single value. Rust has four primary scalar types: integers, floating-point numbers, Booleans, and characters.

</details>

## Day 18

- Finished `3.2. Data Types`.

<details>

<summary>Takeaway tips</summary>

- Rust has two primitive compound types: *tuples* and *array*.
- A `tuple` is a general way of grouping together a number of values with a variety of types into one compound type. Tuples have a *fixed length*: once declared, they cannot grow or shrink in size.
- Unlike a tuple, every element of an `array` must have the same type. Unlike arrays in some other languages, arrays in Rust have a fixed length.

</details>

## Day 19

- Started [3.3. Functions](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html)

<details>

<summary>Takeaway tips</summary>

- In function signatures, you *must* declare the type of each parameter.
- Statements are instructions that perform some action and do not return a value.
- Expressions evaluate to a value.
- Expressions do not include ending semicolons. If you add a semicolon to the end of an expression, you turn it into a statement, and it will then not return a value.

</details>

## Day 20

- Finished [3.3. Functions](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html)
- Finished [3.4. Comments](https://doc.rust-lang.org/book/ch03-04-comments.html)
- Started [3.5. Control Flow](https://doc.rust-lang.org/book/ch03-05-control-flow.html)

<details>

<summary>Takeaway tips</summary>

- You must be explicit and always provide `if` with a **Boolean** as its condition.
- In the `if`, `else if` and `else` blocks, if the types are mismatched, it will throw an error.

</details>