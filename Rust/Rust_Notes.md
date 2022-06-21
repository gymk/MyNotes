# RUST

All the study did with `rustc 1.57.0 (f1edd0429 2021-11-29)` version

- [RUST](#rust)
  - [Rust related documentation](#rust-related-documentation)
  - [Installation](#installation)
    - [System Instlalations](#system-instlalations)
    - [Rust in WSL (Windows)](#rust-in-wsl-windows)
    - [Updating rust](#updating-rust)
    - [Rust Uninstall in WSL](#rust-uninstall-in-wsl)
  - [Project and Library Crates](#project-and-library-crates)
  - [Peformance analysis](#peformance-analysis)
  - [serial_test](#serial_test)
  - [Data Types](#data-types)
  - [Functions](#functions)
  - [Control Flow](#control-flow)
  - [Enumerations (`enum`)](#enumerations-enum)
  - [struct](#struct)
  - [Traits](#traits)
  - [Macros](#macros)
  - [Attributes](#attributes)
  - [Ownership and Borrowing](#ownership-and-borrowing)
  - [Error handling](#error-handling)
  - [Annotations](#annotations)
  - [Other](#other)
    - [Shorthand struct initialization](#shorthand-struct-initialization)
  - [Stud Links](#stud-links)

## Rust related documentation

- Online documentation
  - <https://doc.rust-lang.org/>
- Offline documentation (this is included with rust installation)
  - `rustup doc`
- Cargo Book
  - <https://doc.rust-lang.org/cargo/index.html>

## Installation

- Details
  - `cargo`
    - Build System and Packag Manager tool
      - [Cargo Commands](https://doc.rust-lang.org/cargo/commands/index.html)
  - `rustc`
    - Rust compiler
  - `rustup`
    - Tool to install rust
  - `Cargo.toml`
    - Created when you create a crate
    - File contains some metadata about the project
      - Cargo fill in with information from your environment if it is available
    - Dependencies
      - We will be listing all the dependencies our project has
  - `Cargo.lock`
    - Created by `cargo build`
    - Which keeps track of the exact versions of dependencies that is used in the project

### System Instlalations

- `sudo apt-get install libudev-dev`
- `sudo apt install pkg-config`

### Rust in WSL (Windows)

  ```text
  sudo apt install build-essential
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

  - /home/ubuntu/.rustup
    - Rustup metadata and toolchains will be installed into the Rustup home directory, located at:
    - This can be modified with the RUSTUP_HOME environment variable.
  - /home/ubuntu/.cargo
    - The Cargo home directory located at:
    - This can be modified with the CARGO_HOME environment variable.
  - /home/ubuntu/.cargo/bin
    - The `cargo`, `rustc`, `rustup` and other commands will be added to Cargo's bin directory, located at:
    - This path will then be added to your PATH environment variable by modifying the profile files located at:
      - /home/ubuntu/.profile
      - /home/ubuntu/.bashrc
  ```

### Updating rust

- `rustup update`

### Rust Uninstall in WSL

- `rustup self uninstall`

## Project and Library Crates

- To Create Project
  - `cargo new --bin projectname`
    - This creates a new binary crate called projectname that will produce an ***executable***
- To Crerate Library
  - `cargo new --lib mylib`
    - This create a new library crate called mylib
    - Library are meant to be used for current and other projects
- To build
  - `cargo build`
    - By default cargo builds with very few optimizations turned on
      - This makes compiling does not take very long and produces build with debugging information in them
    - `targer/debug/project exe`
  - `cargo build --release`
    - It builds without debugging information
      - best for performance analysis
    - `target/release/project exe`
- To build and run (most common)
  - `cargo run`
    - This will do both *build* and run the executable
- To check current project/package
  - `cargo check`
    - [This](https://doc.rust-lang.org/cargo/commands/cargo-check.html) will check local package and all of its dependencies for errors.
    - This will compile the packages without performing the final step of code generation, which is faster than running `cargo build`.

## Peformance analysis

- If you planning to do any performance analysis, make sure to build in *`release`* mode by
  - `cargo build --release`

## serial_test

- Creating the project
  - `cargo new serial_test --bin`
  - `cd serial_test`
- Editing code in Windows
  - `code .`
- Running the project
  - `cargo run`
- Publishing
  - Ensure that you
    - **update**
      - your version
      - file exlcusions in
        - *`Cargo.toml`*
        - *`.gitignore`*
  - `cargo package`
  - `cargo publish`
- Running Test
  - `cargo test`
  - `cargo test -- --nocapture`
    - Show program output
  - `cargo test test_name`
    - To run specific test
- Backtrace
  - `RUST_BACKTRACE=1 carggo run`
  - `RUST_BACKTRACE=full carggo run`
- To see dependency graph
  - `cargo tree`

## Data Types

- Primitive Types - <https://doc.rust-lang.org/std/primitive.bool.html#impl-Default>
  - `bool, char, f32, f64, fn, i8, i16, i32, i64, i128, isize, never, pointer, reference, slice, str, tuple, u8, u16, u32, u64, u128, unit, usize`

- Simple
  - Boolean
    - `bool`
      - `true` or `false`
  - Integer
    - Default type is `i32`
    - signed and unsigned
      - Size can be decided
        - signed `i8/i16/i32/i64/i128`
        - unsigned `u8/u16/u32/u64/u128`
    - `isize` and `usize`
      - Size of the pointer (dependent on the architecture)
        - 32 bit architecutre system - 32 bit space
        - 64 bit architecture system - 64 bit space
      - Normally used for indexing into collections or counting items
        - `let a = [100, 200, 300];`
          - An array of integers
        - `let b = a[0];`
          - `0` that is used in above indexing has the type 'usize'
  - Floating Point
    - `f32` and `f64` using 32 and 64 bits respectively
    - Default type - `f64`
  - Character
    - Holds *Unicode scalar value*
    - *Not only ASCII*
    - Characters are written with single quotes `let c = 'b';`
      - Strings use double quotes though
- Compound
  - Tuple
    - To group multiple values together into one type using paranthesis
      - These values don't have to the same type
      - Example: `let mytup = (1, 'c', true)`
    - Indexing typle
      - To get individual items, start accessing from '0'
        - `println!("{} {} {}", mytup.0, mytup.1, mytup 2);`
    - Destructuring
      - `let (x, y, z) = mytup;`
  - Array
    - It is collections in rust, to group multiple values into one value
      - Value MUST be the same type
        - Example: `let a = [0.0, 3.14, -8.7928];`
      - Indexing
        - Use square brakets `[]`
        - Index starts from `0`
      - Length
        - ***Length are fixed*** when you initialize them
          - It *can't get bigger or smaller* even if we have the variable as mutable `mut`
            - `let mut b = [1, 10, 15];`
              - OK - will compile
                - `b[1] += 10;`
              - KO - won't compile
                - `b += 100;`
            - `println!("{:?}", b);`
          - If the sequence of values need to be changes, we can use `Vec`, a type provided by standard library
  - Slice [In-Depth on Slice](Ownership_And_Borrowing.md#Slices)
    - It lets us reference a contiguous subset of data in another data structure
      - Say view of that data structure
    - Example - 1:
      - `let a = [100, 200, 300];`
      - `let b = &a[0..1];`
        - A slice of `i32`'s, `b` references the elements of a starting from index `0` inclusive till index `1` exclusive
        - `println!("{:?", b);` will print `[100]`
        - Note that index `1` is excluded in the slice
    - Example - 2: String slices
      - `let s = "hi";`
        - Here `"hi"` is a `&str` string slice

## Functions

- Defining functions
  - Syntax

    ```rust
    fn name(param1: type1, ...) -> return_type {
      ... body ...
    }
    ```

- if function is not returning any value, leave out `-> return_type`

    ```rust
    fn next_birthday(name: &str, curreng_age: u8) {
      let next_age = current_age + 1;
      println!("Hi {}, on your next birthday, you'll be {}", name, age);
    }
    ```

- Calling functions
  - Syntax
    - `name(arg1, ...);`
      - `next_birthday("Y", 20);`
- Returning values from functions
  - Example:

    ```rust
    fn square1(num: i32) -> i32 {
      num * num
    }

    fn square2(num: i32) -> i32 {
      return num * num;
    }

    fn main() {
      println!("The answer is {} {}", square1(4), square2(5));
    }
    ```

    - In `square1` function, note that last line in the body doesn't have semicolon `;`
      - This tells rust the resulting value of that expression to be returned
    - In `square2` function, we use `return` keyword to explicitly tell that the expression value to be returned.

## Control Flow

[Control Flow related Notes](Control_Flows.md)

- This covers
  - `if` / `else if` / `else`
  - `loop`, `while`, `for`
  - `match`

## Enumerations (`enum`)

[Enumeration related Notes](Enumerations.md)

## struct

[`struct` notes](Structs.md)

## Traits

[Traits Notes](Traits.md)

## Macros

[Macros Notes](Macros.md)

## Attributes

- To Understand

- `derive`

## Ownership and Borrowing

[Ownership and Borrowing](Ownership_And_Borrowing.md)

## Error handling

[Error handling in Rust](Error_Handling.md)

## Annotations

[Annotation Notes](Annotations.md)

## Other

### Shorthand struct initialization

- if arguments and files having same name, then we can use shorthand initialization

  ```rust
  fn new(name: String, number: u8, position: HockeyPosition) -> HockeyPlayer {
    HockeyPlayer {
      name: name,
      number: number,
      position: position,
      goals_ytd: 0,
    }
  }
  ```

  ```rust
  fn new(name: String, number: u8, position: HockeyPosition) -> HockeyPlayer {
    HockeyPlayer {
      name,
      number,
      position,
      goals_ytd: 0,
    }
  }
  ```

## Stud Links

- <https://github.com/rylev/learn-rust-the-hard-way>
  - Some example programs with detailed comments in it to understand the language
- <http://cliffle.com/>
  - C++ and Rust together - explained well
- <https://rustwasm.github.io/docs/book/game-of-life/introduction.html>
  - How to use
    - Rust, WebAssembly and JavaScript **together**
