# Annotations in Rust

- `#[dervice(debug)]`
  - Example

      ```rust
      #[derive(Debug)]
      struct Bucket {
          liters: u32,
      }
      ```

    - This annotation on the struct definition enables us to print out the values of `Bucket` for inspection later
