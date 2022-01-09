# Macros

- `println!`
  - It prints to *`stdout`*
- `dbg!`
  - Used to print values using `Debug` trait
  - It prints to *`stderr`*
  - Will use ***pretty debug formatting*** for output
  - This macro
    - takes owndership of an expression,
    - prints the file and line number of where that `dbg!` macro call occurs,
    - result value of the expression that is given to the macro
    - and then returns the ownership of the value.
