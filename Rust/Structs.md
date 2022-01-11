# `struct` Structures

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
      - These are strucuts that have '*a name for the whole type*' but don't have name for their fields
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
            - Example: THIS WON'T COMPILE

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
      - Structs actually don't need to have any fields
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
  - Since rust suppoprts `enum variants`, we can have field names assigned to it
    - Unlike `struct` (e.g., tuple struct) which uses paranthesis `()`; Instead of paranthese we use flower brackets `{}`
      - That is, While creating the `enum` variable, we use flower brackets

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

  - In this example, `Sundial`, `Digital` and `Analog` are enum variants
    - Its equivalent types are defined inside flower brackets `{}` which has its own *field names*
  - In `struct` we will just have "field_name : data_type", and neither paranthesis nor flower brackets we use while defining the fields

## Defining struct

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

- Using `new()` method ***if this associated method is defined for that struct***
- If *`new`* not associated method is not available, then use the following way for instantiation:

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

- When the methods are useful?
  - When you want to add a behaviour to the custom data type that you defined (and that behavior ONLY applicable to that data type)\
    - E.g., a coach won't had a position in the game
- Syntax for defining a method
  - Defined within an `impl` block
    - Define your function within `impl struct_name` `{}` block
      - First parameter of the function is always `self`
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
