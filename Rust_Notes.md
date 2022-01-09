# RUST

## Rust related documentation

- Online documentation
  - <https://doc.rust-lang.org/>
- Offline documentation (this is included with rust installation)
  - `rustup doc`

## Installation

- Details
  - `cargo`
    - Build System and Packag Manager tool
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

## Peformance analysis

- If you planning to do any performance analysis, make sure to build in *`release`* mode by
  - `cargo build --release`

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
  - Slice
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

- `if` / `else if` / `else`

    ```rust
    if expression {
      ...code...
    } else if expression {
      ...code...
    } else {
      ...code...
    }
    ```

  - Example:

    ```rust
    fn discount(day_of_month: u8) {
      let amount = if day_of_month % 2 == 0 {
        50
      } else {
        10
      }

      println!("Your discount is {}!", amount);
    };
    ```

    - Like functions, we can get a value from an `if` or `else` block by leaving the semicolon `;` off the last expression
      - But we ***do need*** the `;`  to end of the variable assignment statement
    - When you return a value from an `if else` expression for a variable, there must be an `else` clause so that the variable always gets a value.
      - And the types of the values MUST all be same

- `loop`, `while`, `for`
  - `loop`

    ```rust
    fn main() {
      loop {
        println!("Hello World!!!");
      }
    }
    ```

    - We can use `break` to stop the loop

      ```rust
      fn main() {
          let mut i = 0;
          loop {
              i += 1;
              if i > 10 {
                  break;
              }
              println!("Hi!!!");
          }
      }
      ```

      ```rust
      use std::io;

      fn main() {
          loop {
              println!("What's the secre word?");
              let mut word = String::new();
              io::stdin().read_line(&mut word).expect("Failed to read line");

              // println!("You entered [{}]", word.trim());
              if word.trim() == "rust" {
                  break;
              }
          }

          println!("You know the secret word! Please proceeed!");
      }
      ```

  - `while`

    ```rust
    use std::io;

    fn main() {
        let mut word = String::new();
        while word.trim() != "rust" {
            println!("What's the secre word?");
            word = String::new();
            io::stdin().read_line(&mut word).expect("Failed to read line");
            // println!("You entered [{}]", word.trim());
        }

        println!("You know the secret word! Please proceeed!");
    }
    ```

  - `for`
    - Unlike other lanaguges, you don't need a *index* into the collections

      ```rust
      fn main() {
          for i in 1..11 {
              println!("Current series is [{}]", i);
          }
      }
      ```

      - In above range, it will start printing from `1` (inclusive) till `10` (excluding 11)

- `match`
  - Similar to
    - `if / else if / else`
    - `switch`
    - `case`
  - Better because of
    - Pattern matching
    - Exhaustiveness checking
  - Pattern Matching
    - Avaialble in multiple places
      - Tuple destructuring `let (x, y, z) = mytup;`
    - We specify a list of patterns to test a value against an expression, `match` expression tests the value against each pattern and stop if it finds a matching one.

      ```rust
      let x = 3;
      match x {
        1 => println!("Option 1 selected"),
        2 => println!("Option 2 selected"),
        3 => println!("Option 3 selected"),
        _ => println!("Some other option selected, plesae try again"),
      }
      ```

    - `_` in `match` is a ***catch-all*** that will match any value (like last `else` statement in `if`)
      - A lot of different ideas can be expressed with pattern matching

        ```rust
        fn main() {
          let die1 = 1;
          let die2 = 5;

          match (die1, die2) {
            (1, 1) => println!("Snake eyes! Go back to the beginning."),
            (5, _) | (_, 5) => {
              println!("You rolled at least one 5!");
              println!("Move and then roll again!");
            },
            _ => println!("Move your piece!"),
          }
        }
        ```

  - Exhaustive checking
    - `match` must be exhaustive and cover every case (otherwise compiler will throw error to fix the bug)
      - This prevents bugs that can be caused by forgetting to handle a situation
    - Following code won't compile, as all cases are not covered

      ```rust
      fn main() {
        let is_confirmed = true;
        let is_active = false;

        match (is_confirmed, is_active) {
          (true, true) => println!("This account is in good standing"),
          (false, true) => println!("You need to confirm your account!"),
          (false, false) => println!("This account will be deactivated"),
        }
      }
      ```

      - Can be fixed by `_ => {},`
        - Note that in above statement, we say that for rest of the cases, we do nothing

## Enumerations (`enum`)

- What is enum in Rust
  - A way to express that a value can be one out of a finite set of possible values
  - `enum` is one of the way to define custom types in Rust (`struct` are the other)

- When enums are useful
  - Example: A thread state machine (idle, ready, running)
  - Properties of enum
    - Can only be one value at a time
      - Can be in one of the states
    - Can list (enumerate) all possible values
      - Can list the finite number possible states - no need to define new states
- Syntax for defining an enum

    ```rust
    enum Clock {
      Sundial,
      Digital,
      Analog,
    }
    enum HockeyPosition {
      Center,
      Wing,
      Defense,
      Goalie,
    }
    ```

  - For e.g., `Sundial`, `Digital` and `Analog` are called *enum variants*
  - Both enum names and enum variants should be in ***camel case***
- Syntax for using an enum
  - Example: `HockeyPosition::Center`
- Defining enum variants that hold data
  - This is something new when compared to other languages C/JavaScript, which can't hold additional data
  - Below is an example reprensets different kind of data that we can have in `enum`

    ```rust
    enum Clock {
      Sundial(u8),
      Digital(u8, u8),
      Analog(u8,u8, u8),
    }
    ```

    - Sundial (which will have only hour value), Digital (which will have only Hours and Minutes), Analog (which will have hours, mintues and seconds)
  - enum variants can hold multiple pieces of data
    - The paranthesis are like a `tuple`
  - enum variants can also have named fields and look like a `struct`
- Enums and match expressions

  ```rust
    enum Clock {
      Sundial(u8),
      Digital(u8, u8),
      Analog(u8, u8, u8),
    }

    fn tell_time(clock: Clock) {
      match clock {
        Clock::Sundial(hours) =>
          println!("It is about {} o-clock", hours),
        Clock::Analog(hours, minutes, seconds) =>
          println!("It is {} minutes and {} seconds past {} o'clock", minutes, seconds, hours),
        Clock::Digital(hours, minutes) =>
          println!("It is {} minutes past {}", minutes, hours),
      }
    }

    fn main() {
      tell_time(Clock::Analog(11, 36, 20)); 
      tell_time(Clock::Digital(11, 36)); 
      tell_time(Clock::Sundial(11)); 
    }

    // It is 36 minutes and 20 seconds past 11 o'clock
    // It is 36 minutes past 11
    // It is about 11 o-clock
  ```

## struct

- A type composed of other types.

- What is a `struct`
  - A custom data type that group related data together
- When structs are useful
  - When we need to add more attributes to describe an object - `employee`
    - `name`, `age`, `department`, ...
    - These employee related data can be grouped together as a `struct
- Syntax for defining a struct

  ```rust
  struct HockeyPlayer {
    name: String,
    number: u8,
    position: HockeyPosition,
    goals_ytd: u8,
  }
  ```

  - `struct` name should be in ***camel case***
  - field names should be in ***snake case***

- Syntax for using a struct

  ```rust
  fn main() {
      let player = HockeyPlayer {
          name: String::from("BT1"),
          number: 12,
          position: HockeyPosition::Center,
          goals_ytd: 6,
      };

      println!(
          "player {} has scored {} goals till now",
          player.name, player.goals_ytd
      );
  }
  // player BT1 has scored 6 goals till now
  ```

  - to change value, add `mut`
    - `let mut player = HockeyPlayer {`
    - `player.goals_ytd += 1;`

- Other forms of structs
  - Tuple structs and unit structs
    - Tuple structs
      - These are strucuts that a name for the whole type but don't have name for their fields
        - `struct Triangle(u32, u32, u32);`
        - `let triangle1 = Triangle(3, 4, 5);`
      - These behave similarly to tuples
        - Can be accessed with a `.` and an index (starting from `0`)

          ```rust

          // 3 sides of the triangle
          struct Triangle(u32, u32, u32);

          fn is_equilateral(triangle: Triangle) -> bool {
            triangle.0 == triangle.1 && triangle.1 == triangle.2
          }

          fn main() {
            let triangle1 = Triangle(10, 20, 30);
            println!("Is it equilateral - {}", is_equilateral(triangle1));
          }

          // Is it equilateral - false
          ```

      - Uses
        - For **strict type checking**
          - Following won't work eventhough number of arguments and its types are same
            - `let sizes = (5, 5, 5);`
            - `is_equilateral(nums);`
          - This tuple struct ensures that we are passing the right units of arguments
          - That is we are giving the tuple a name that makes this new type incompatible with other tuples
        - Used for **newtype pattern**
          - An existing type is wrapped in a tuple struct with one element to add meaning
            - Example:

              ```rust
              struct Meters(u8);

              fn add_distances(d1: Meters, d2: Meters) -> Meters {
                Meters(d1.0 + d2.0)
              }

              fn main() {
                let dist1 = Meters(3);
                let dist2 = 7;
                let dist3 = add_distances(dist1, dist2);
              }
              ```

              - Above code won't compile, we are passing `u8` against `Meters` which are incompatible tupes
              - This newtype pattern prevents bugs caused by accidently using values that are in the wrong units
    - Unit structs
      - Structs actually don't need to have any fileds
      - Structs without fields are called **Unit Structs**
      - Use
        - Eventhough these is no fields, **we can define methods on them**

      ```rust
      struct MyStruct;

      fn main() {
        let mystruct = MyStruct;
      }
      ```

- Enum variants that look like structs
  - Instead of paranthese we use flower brackets `{}`
  - While create the variable also, we use flower brackets

  ```rust
  enum Clock {
    Sundial { hours: u8 },
    Digital { hours: u8, minutes: u8 },
    Analog { hours: u8, minutes: u8, seconds: u8 },
  }

  fn main() {
    let clock = Clock::Analog {
      hours: 11,
      minutes: 36,
      seconds: 20
    };
  }
  ```

### Defining struct

- Mutability
  - Only if fields are defined as mutable `mut`
    - `example_struct.field2 = "Test".to_string();`
- 3 flavors
  - Struct with named fields
    - Each field defined has a name and value.
      - Can be access like `example_struct.field1`

    ```rust
    struct Regular {
      mut field2 : String,
      pub field3 : bool
    }
    ```

  - Tuple Structs
    - Similar to field strucuts
      - But do not have names
    - These are used like tuples.
    - `let TupleStruct(x, y) = foo;`
    - Accessing variables
      - `foo.0`
      - `foo.1`

    ```rust
    struct Tuple(u32, String);
    ```

  - Unit Structs
    - Commonly used as *markers*.
    - They have a size of *`zero`* bytes
    - Unlike empty enums, these can be instantiated
      - Making them isomorphic to the unit type `()`
    - Unit struct is useful
      - When you need to implement a trait on something, but don't need to store any data inside it

    ```rust
    struct Unit;
    ```

### Instantiation

Many ways

- Using `new()` construct if this construct is defined for that struct
- If *`new`* not available, then use the below method:

    ```rust
    let example = foo {
      field1: 40.20,
      field2: "blah".to_string(),
      etc: true
    }
    ```

## Methods

- methods are similar to functions
- methods are defined for `enum` and `struct` that define the behavior

- When methods are useful
  - When you want to add a behaviour to the custom data type that you defined (and that behavior ONLY applicable to that data type)\
    - E.g., a coach won't had a position in the game
- Syntax for defining a method
  - Define your function with `impl` `struct_name` `{}`
    - First parameter of method is always `self`
- Syntax for calling a method
  - Call variable instance with `.` and the name of the methods, with required arguments

```rust
enum HockeyPosition {
  Center,
  Wing,
  Defense,
  Goalie,
}
struct HockeyPlayer {
  name: String,
  number: u8,
  position: HockeyPosition,
  goals_ytd: u8,
}

impl HockeyPlayer {
  fn shoot_puck(self, seconds_remaing: u16) {
    if seconds_remaing < 300 {
      match self.position {
        HockeyPosition::Center => println!("Goal!"),
        _ => println!("Miscc!")
      }
    } else {
      println!("Goal!")
    }
  }  
}

fn main() {
  let mut player = HockeyPlayer {
      name: String::from("BT1"),
      number: 12,
      position: HockeyPosition::Center,
      goals_ytd: 6,
  };

  player.shoot_puck(500);
}
```

### Associated Methods

- Defined within an `impl` block
  - These are defined within an `impl` block on a `struct` or an `enum`, the same as methods
    - These are behaviours associated with that type
- Don't take `self` as a parameter
  - These associated function don't take `self` argument
    - But still the behaviour defined by this associated function is related to that type
- Use
  - Commonly *used to create instances*
    - `self` won't be available while creating an instace, hence, associated functions

  ```rust
  enum HockeyPosition {
    Center,
    Wing,
    Defense,
    Goalie,
  }
  struct HockeyPlayer {
    name: String,
    number: u8,
    position: HockeyPosition,
    goals_ytd: u8,
  }

  impl HockeyPlayer {
    fn shoot_puck(self, seconds_remaing: u16) {
      if seconds_remaing < 300 {
        match self.position {
          HockeyPosition::Center => println!("Goal!"),
          _ => println!("Miscc!")
        }
      } else {
        println!("Goal!")
      }
    }  
    fn new(name: String, number: u8, position: HockeyPosition) -> HockeyPlayer {
      HockeyPlayer {
        name: name,
        number: number,
        position: position,
        goals_ytd: 0,
      }
    }
  }

  fn main() {
    let mut player = HockeyPlayer::new(
        String::from("BT1"),
        12,
        HockeyPosition::Center
    );

    player.goals_ytd = 7;

    player.shoot_puck(500);
  }
  ```

### Methods that read data vs. methods that write data

- `&self` to read data
  - It borrows the instance (not taking ownership)
  - It is for methods that don't change anything about `self`'s values
- `&mut self` to write data

```rust
enum HockeyPosition {
    Center,
    Wing,
    Defense,
    Goalie,
}
struct HockeyPlayer {
    name: String,
    number: u8,
    position: HockeyPosition,
    goals_ytd: u8,
}

impl HockeyPlayer {
    fn shoot_puck(&mut self, seconds_remaing: u16) {
        if seconds_remaing < 300 {
            match self.position {
                HockeyPosition::Center => {
                    self.goals_ytd += 1;
                    println!("Goal!")
                }
                _ => println!("Miss!"),
            }
        } else {
            self.goals_ytd += 1;
            println!("Goal!")
        }
    }
    fn new(name: String, number: u8, position: HockeyPosition) -> HockeyPlayer {
        HockeyPlayer {
            name,
            number,
            position,
            goals_ytd: 0,
        }
    }
}

fn main() {
    let mut player = HockeyPlayer::new(String::from("BT1"), 12, HockeyPosition::Center);

    player.goals_ytd = 7;

    player.shoot_puck(900);
    player.shoot_puck(500);

    println!(
        "player {} has scored {} goals till now",
        player.name, player.goals_ytd
    );
}

// Goal!
// Goal!
// player BT1 has scored 9 goals till now
```

## Traits

### Derivable Traits

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

## Macros

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

## Attributes

- To Understand

- `derive`

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
