# learning-rust
A repository with Rust lessons.

## 1. Using cargo
You can do several things with cargo, like:
- Create a new project: `cargo new project_name`
- Build the project: `cargo build`
- Run the project: `cargo run`
- Check the project: `cargo check`
- Test the project: `cargo test`
- Build the project for release: `cargo build --release`
- Generate the project documentation: `cargo doc`
- Publish the project to crates.io: `cargo publish` (you need to have an account on crates.io and set the `CARGO_REGISTRY_TOKEN` environment variable with the token)
- Install a binary: `cargo install binary_name`
- Uninstall a binary: `cargo uninstall binary_name`
- Update the dependencies: `cargo update`

More info can be found at [The Cargo Book](https://doc.rust-lang.org/cargo/).


## 2. Variables
By default, variables are immutable in Rust. To make them mutable, you need to use the `mut` keyword.

```rust
let x = 5; // Immutable
let mut y = 10; // Mutable
```

## 3. Data types
There are four primary scalar types in Rust:
- Integers: `i8`, `i16`, `i32`, `i64`, `i128`, `u8`, `u16`, `u32`, `u64`, `u128`, `isize`, `usize`
  - `isize` and `usize` depend on the kind of computer your program is running on: 64 bits if you're on a 64-bit architecture and 32 bits if you're on a 32-bit architecture.
- Floating-point numbers: `f32`, `f64`
- Booleans: `bool`
- Characters: `char`

Example of `isize` and `usize`:

```rust
let x: isize = 10;

// Print size of x
println!("Size of x: {}", std::mem::size_of_val(&x));
```