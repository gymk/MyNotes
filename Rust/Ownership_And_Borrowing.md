# Owernership and Borrowing

- This ownership and borrowing is a memory management concept that provides memory safety gurantee in Rust

## Ownership

- Definition of ownership
  - Rust's strategy of managing memory
- How ownership differs from:
  - Manual memory management in C
  - Garbage collection in Ruby
- Example: The String data type
  - Moving ownership
    - From one function to another
  - Cloning data
    - Basically duplicating it, to have two separate owners
- Execise with functions and strings

### What is ownership

- It is Rust's strategy for managing data and memory and preventing common problems
- Each piece of data has one owning variable, the memory owner
- Owner is responsible for cleaning up that data and no one else
  - Cleanup happens when the owner goes out of scope
- The owner decides on mutability

### How ownership differs `C` / `Ruby` / `Rust`

- `C`
  - Code has to explicitly has to perform both allocation and deallocation
    - `malloc`, `free`
  - Advantage
    - Complete control over your program memory and how much it uses
  - Disadvantage
    - C doesn't prevent you from messing with pointers
      - Use after free
        - dangling pointers
      - Memory leaks
        - if deallocation is not done by the program
      - Double free
        - Two parts of your program tries to dealloate the same data
  - These disadvantages cause major security flaws and crashes
    - Example: Heartbleed vulnerability

    ```c
    #include <stdio.h>
    #include <stdlib.h>

    typedef struct tagNode {
      char cLeftChannel;
      char cRightChannel;
      char cValue;
    } NODE;

    int main(int argc, char *argv[]) {
      /* manually allocate */
      NODE * pNode = malloc(sizeof(NODE));
      
      pNode->cLeftChannel = 0;
      pNode->cRightChannel = 0;
      pNode->cValue = 10;

      /* manually deallocate */
      free(pNode);

      return 0;
    }
    ```

- `Ruby`
  - It takes care of managing memory
    - By garbage collector
      - It is a component running alongside your program, cleaning up memory that you are done wtih
    - Advantage
      - No need to think about managing memeory at all
      - GC keeps trackof which memory is still in use and cleans up any data that your program is done with
      - And we don't have the disadvantages listed for 'C"
    - Disadvantage
      - Down side of GC is you lose control and performance
        - When GC runs,
          - It takes up computer resources that your program could be using
          - Your program will also likely use more memory than it strictly needs because of the GC (Because GC cleans up only if it is sure that your program is done)
- `Rust`
  - It tries to get the **best of both worlds**
    - Ownership gives you *control over memory allocation*
      - and the associated performance by cleaning up data ***automatically*** when owner goes out of scope
  - We *can't mess up with memory acess*, and
  - We *won't be using memory longer* than we strictly need to

## Example: The String data type

### String

- It is a type in Rust that has data that needs to be cleaned up when it goes out of scope
- It tracks
  - How much space is allocated
  - how much of that space is used
  - And the UTF-8 data
- When the owner of the string goes out of scope
  - The UTF-8 data needs to be cleaned up

- Creating String

    ```rust
    fn main() {
        let a = String::from("Hi");
        println!("Salutation string {}", a);
    }
    ```

  - `let a = String::from("Hi");`
    - String allocates memory to hold the text "hello"
  - `println!("Salution string {}", a);`
    - It uses the value of `a` for printing
  - When code reaches the closing curly braces `}`
    - At this point, variable `a` goes out of scope
    - The data allocated for `a` is deallocated automatically

  ```rust
  fn main() {
      let mut a = String::from("Hi");
      a.push_str(", Good Morning!!!");
      println!("Salutation string {}", a);
  }

  // Salution string Hi, Good Morning!!!
  ```

  - data can be modified dynamically (variable needs to be mutable `mut`)
    - `a.push_str(", Good Morning!!!");`
      - This will allocate more memory if necessary

## Moving Ownership

- By default, for non-primitive types, rust moves the ownership
- Between variables

    ```rust
    fn main() {
        let a = String::from("Hi");
        let t = a;
        println!("Salution string {}", t);
    }

    // Salution string Hi
    ```

  - `let t = a;`
    - This will move ownership of the string `a` to `t`
      - After this, the variable `a` is no longer available for use
- Between functions

  ```rust
  fn greet(s: String) {
      println!("Hi, {}", s);
  }

  fn main() {
      let a = String::from("Good Morning");
      greet(a);
  }

  // Hi, Good Morning
  ```

  - function `greet` takes ownership of string and prints it out
  - The string `s` gets cleaned-up at the end of function `greet`
- Transferring ownership from function

  ```rust
  fn smiley_open_hands() -> String{
      String::from("🤗")
  }

  fn main() {
      let s = smiley_open_hands();
      println!("String val: {}", s);
  }

  // String val: 🤗
  ```

- Here the ownership of the string created in `smiley_open_hands` is transferred to the function caller
  - It is not cleanedup at the end of the function `smiley_open_hands`
  - Instead ownership is transferred to `a` and it is cleaned-up at the end of `main` when `a` goes out of scope

## Cloning data

- Sometimes we need to keep the ownership of the data but also need to give ownership of the data to another piece of code
  - This is done by *cloning*
- `let t = a.clone();`
  - Cloning makes
    - a deep copy of the allocated memory
    - There is two copies of data owned by each owner `a` and `t` respectively
      - Now each owner has the responsibility to clean up their data
- Cloning is one of the way to overcome errors arised from ownership
  - But we may end up using more memory than we strictly needed

    ```rust
    fn greet(s: String) {
        println!("Hi, {}", s);
    }

    fn main() {
        let a = String::from("Good Morning");
        greet(a.clone());
        println!("Using a again: {}", a);
    }

    // Hi, Good Morning
    // Using a again: Good Morning
    ```

## Fun with functions and strings

```rust
fn main() {
    let s = String::from(:book);

    // Add code here that calls the pluralize function

    println!(
        "I have one {}, you have two {}",
        s,
        you_add_something_ehere,
    );
}

// Add appropriate parameters, return values, and implementations to this function
fn pluralize() {}
```
