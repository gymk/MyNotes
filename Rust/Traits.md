# Traits

## Derivable Traits

- `Debug` trait
  - `{}`
    - Inside `println!`
      - Invokes `std::format::Display` trait
  - `"{:?}"`
    - Inside `println!`
      - Invokes `std::format::Debug` trait
      - If `Debug` trait is defined it will use that
      - Otherwise, we need to add the **`attribute`** *`#[derive(Debug)]]`*
  - `"{:#?}"`
    - To have pretty print
    - Can be used in `println!` trait
    - Used by `dbg!` trait as well

- `PartialEq`
- `PartialOrd`
- `Clone`
- `Copy`
- `Hash`
- `Default`
- The pointer trait
  - `"{:p}"`
    - Formats the memory location, as hexadecimal

      ```rust
      let x = &42;

      let address = format!("{:p}", x); // this produces something like '0x7f06092ac6d0'
      ```

- The `UpperExp` trait
  - `"{:E}"`
    - Formats its output in scientific notation with an upper-case `E`
- The `UpperHex` trait
  - `{:X} // 2A`
    - Formats its output as a number in hexademical, with `A` to `F` in upper-case
    - For primitive signed integers (`i8` to `i128` and `isize`), negative values are formatted as the two's complement representation
    - Alternate flag `#`, adds a `0x` in front of the output
      - `{:#X} // 0x2A`
