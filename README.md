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
