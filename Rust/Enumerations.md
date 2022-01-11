# `enum` Enumerations

- What is enum in Rust
  - A way to express that a value ***can be one out of a finite set of possible values***
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
