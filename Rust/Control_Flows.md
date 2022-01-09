# Control Flows in Rust

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
