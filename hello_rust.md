## **1. Variables and Mutability**

In Rust, variables are immutable by default. If you want to change the value of a variable, you must declare it as mutable with `mut`.

```rust
fn main() {
    let x = 5; // Immutable variable
    println!("The value of x is: {}", x);

    let mut y = 10; // Mutable variable
    println!("The value of y is: {}", y);
    y = 15;
    println!("The new value of y is: {}", y);
}
```

---

## **2. Data Types**

Rust is statically typed, meaning the data type of a variable must be known at compile time.

### Primitive Data Types:
- Integer: `i8`, `i16`, `i32`, `i64`, `u8`, `u16`, etc.
- Floating-point: `f32`, `f64`
- Boolean: `bool`
- Character: `char`
- Tuple: Fixed-size collection
- Array: Fixed-size collection of the same type

```rust
fn main() {
    let int: i32 = 10; // Integer
    let float: f64 = 20.5; // Float
    let is_active: bool = true; // Boolean
    let letter: char = 'A'; // Character

    let tuple: (i32, f64, char) = (500, 6.4, 'Z'); // Tuple
    println!("Tuple values: {} {} {}", tuple.0, tuple.1, tuple.2);

    let array: [i32; 3] = [1, 2, 3]; // Array
    println!("Array element: {}", array[0]);
}
```

---

## **3. Ownership and Borrowing**

Rust uses ownership to manage memory. Every value in Rust has an owner, and there can only be one owner at a time. When the owner goes out of scope, Rust automatically cleans up the value.

### Ownership Example:
```rust
fn main() {
    let s1 = String::from("Hello");
    let s2 = s1; // Ownership is moved to s2; s1 is no longer valid
    println!("{}", s2); // Works
    // println!("{}", s1); // Error: s1 is no longer valid
}
```

### Borrowing Example:
```rust
fn main() {
    let s = String::from("Hello");
    let len = calculate_length(&s); // Borrowing
    println!("The length of '{}' is {}.", s, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

---

## **4. Functions**

Functions in Rust are defined using the `fn` keyword. The type of parameters and the return value must be explicitly defined.

```rust
fn main() {
    let result = add(5, 3);
    println!("The sum is: {}", result);
}

fn add(x: i32, y: i32) -> i32 {
    x + y // No semicolon: Expression returns the value
}
```

---

## **5. Control Flow**

### If-Else:
```rust
fn main() {
    let number = 10;

    if number > 5 {
        println!("Number is greater than 5");
    } else {
        println!("Number is less than or equal to 5");
    }
}
```

### Loops:
```rust
fn main() {
    // Infinite loop
    let mut count = 0;
    loop {
        count += 1;
        if count == 5 {
            break;
        }
    }

    // While loop
    while count > 0 {
        println!("Count: {}", count);
        count -= 1;
    }

    // For loop
    let arr = [10, 20, 30];
    for val in arr.iter() {
        println!("Value: {}", val);
    }
}
```

---

## **6. Structs**

A `struct` is a custom data type that lets you group related values.

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    println!("Area: {}", calculate_area(&rect));
}

fn calculate_area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
```

---

## **7. Enums**

Enums allow you to define a type by enumerating its possible values.

```rust
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn main() {
    let dir = Direction::Up;

    match dir {
        Direction::Up => println!("Going up!"),
        Direction::Down => println!("Going down!"),
        Direction::Left => println!("Going left!"),
        Direction::Right => println!("Going right!"),
    }
}
```

---

## **8. Pattern Matching**

Rust's `match` expression is powerful for handling multiple conditions.

```rust
fn main() {
    let number = 3;

    match number {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        _ => println!("Something else"), // Default case
    }
}
```

---

## **9. Error Handling**

Rust has built-in support for error handling using `Result` and `Option`.

### Result:
```rust
use std::fs::File;

fn main() {
    let file = File::open("hello.txt");

    match file {
        Ok(f) => println!("File opened successfully: {:?}", f),
        Err(e) => println!("Failed to open file: {}", e),
    }
}
```

### Option:
```rust
fn main() {
    let numbers = vec![1, 2, 3];
    let first = numbers.get(0);
    match first {
        Some(&val) => println!("First number: {}", val),
        None => println!("No number found"),
    }
}
```

---

## **10. Cargo (Rust's Build System)**

### Create a new project:
```bash
cargo new my_project
cd my_project
```

### Build and run:
```bash
cargo build
cargo run
```

### Add dependencies:
Edit `Cargo.toml` to include dependencies:
```toml
[dependencies]
rand = "0.8"
```

---

## Example: Full Application

Here’s a simple example of a guessing game using Rust:

```rust
use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
```

---

### Why Learn Rust?

1. **Safety**: Rust prevents null pointer dereferencing, dangling pointers, and data races.
2. **Performance**: Comparable to C/C++ with zero-cost abstractions.
3. **Concurrency**: Built-in tools for concurrent programming.
4. **Growing Ecosystem**: Ideal for systems programming, web development, and more.
