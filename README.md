# String Calculator

A string calculator in Rust, built using **Test-Driven Development (TDD)** from the ground up. This library provides basic operation for addition.

## Technologies Used

- [Rust](https://www.rust-lang.org/)
- Built-in testing with `cargo test`

## Installation

To use this library in your Rust project:

1. Clone the repository:

   ```bash
   git clone https://github.com/DeveshMhatre/string_calculator.git
   cd string_calculator
   ```

2. Build the project:
   ```bash
   cargo build
   ```

### Example API

```rust
pub fn add(numbers: String) -> i64;
```

## Testing

Since this project was developed using **TDD**, all core functionality is thoroughly tested.

Run all tests using:

```bash
cargo test
```

Example test output:

```bash
running 8 tests
test tests::comma_and_newline_separated_digits ... ok
test tests::comma_separated_digits ... ok
test tests::negative_number - should panic ... ok
test tests::empty_string ... ok
test tests::not_a_number - should panic ... ok
test tests::single_digit ... ok
test tests::newline_as_input - should panic ... ok
test tests::specify_delimiter_at_start ... ok
```

You can view the tests in the `src/lib.rs` file under the `#[cfg(test)]` module.

## Project Structure

```
.
├── Cargo.toml
└── src
    └── main.rs
    └── lib.rs
```

## Author

Created by [Devesh Mhatre](https://github.com/DeveshMhatre)
