# mal-rust

MAL is a Joel Martin project consisting in a set of instructions to
build a LISP interpreter in any Turing-complete programming
language. This projects implements a LISP interpreter following the
instructions from the MAL
[repository](https://github.com/kanaka/mal) but totally adapted to
Rust.

MAL original project has lot of impressive generic code to allow a
uniform experience implementing the intepreter in any programming
language, but this is out of scope for this project. As all that code
is excessive for a single implementation, this project builds a
language specific version of MAL. You can think of it as a
implementation of MAL as if it were intended to be written only in a
programming language.

The project follows a linear path, tagging the master branch with the
steps, but working only with one source code (not one for step). The
testing is performed exclusively in Rust and can be run using `cargo`.

## Usage

To use `mal-rust`, compile and run with `cargo`
```bash
cargo build --release
cargo run
```

or install it and run it by its name
```bash
mal-rust
```

## Tests

Run tests with
```bash
cargo test
```
