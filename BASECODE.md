rust-tuts

Overview
- This repository contains two Rust projects: matches and hello_world. Each project demonstrates basic Rust concepts such as enums, structs, control flow, and variable manipulation. Both projects use the Rust 2024 edition and have no external dependencies.

Project Structure
- matches/
  - Description: Contains the matches project
  - Details: Demonstrates enum usage and struct definitions
- hello_world/
  - Description: Contains the hello_world project
  - Details: Shows basic Rust syntax including functions, conditionals, and loops

matches Project Details
- Source Code (src/main.rs)
  - Enum Definition
    - Code: enum Matches { Chelsea, ManUnited, France }
    - Description: Defines an enum with three variants representing different matches
  - Struct Definition
    - Code: struct Car { model: String, engine: String, price: i32 }
    - Description: Defines a struct to represent car properties
  - Main Function
    - Code: fn main() { println!("Hello, world!"); let match_ = Matches::Chelsea; match match_ { Matches::Chelsea => println!("Chelsea"), Matches::ManUnited => println!("ManUnited"), Matches::France => println!("France"), }; let lambo = Car { model: "Lambo".to_string(), engine: "V8".to_string(), price: 500_000 }; let (x, y) = (10, 20); let is_lambo = { if lambo.model.to_lowercase() == "lambo".to_string().to_lowercase() { true } else { false } }; println!("{}", lambo.model); println!("{}", x); println!("{}", is_lambo); }
    - Description: Demonstrates enum pattern matching, struct instantiation, tuple destructuring, conditional expressions, and printing various values
- Cargo Configuration (Cargo.toml)
  - Package Name: matches
  - Version: 0.1.0
  - Edition: 2024
  - Dependencies: None specified

hello_world Project Details
- Source Code (src/main.rs)
  - Function Definition
    - Code: fn is_grt50(x: i32) { if x > 5 { println!("Yes greater than 50"); } else { println!("No it is less than 50"); } }
    - Description: Defines a function that checks if a number is greater than 5. Note that the comment suggests it should check for greater than 50, but the code checks for greater than 5.
  - Main Function
    - Code: fn main() { let mut name = "DUDE"; println!("Hello, rust {name}"); name = "Hector"; println!("Hello, rust {name}"); is_grt50(40); let mut current_iter = 0; while current_iter < 10 { if current_iter > 51 { println!("Done at {current_iter}"); break; } current_iter += 1; println!("Current loop count:{current_iter}"); } }
    - Description: Demonstrates mutable variable usage, string interpolation, function calls, while loops with conditional breaks, and printing loop progress
- Cargo Configuration (Cargo.toml)
  - Package Name: hello_world
  - Version: 0.1.0
  - Edition: 2024
  - Dependencies: None specified

Building and Running Instructions
- Step 1: Navigate to the project directory
  - Command: cd matches or cd hello_world
- Step 2: Build the project
  - Command: cargo build
- Step 3: Run the project
  - Command: cargo run

Additional Notes
- Note 1: The matches project demonstrates Rust's enum and struct features along with pattern matching.
- Note 2: The hello_world project shows basic Rust syntax including variables, functions, conditionals, and loops.
- Note 3: Both projects use the Rust 2024 edition.
- Note 4: Neither project has external dependencies.
- Note 5: The code contains some apparent inconsistencies, such as the >5 versus >50 comparison, that might need review.