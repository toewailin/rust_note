use std::fs::File;

fn main() {
    println!("hello rust");
    let x = 5; //immutable
    println!("the value of x is: {}", x);
    let mut y = 6; //mutable
    println!("the value of y is: {}", y);
    y = 10;
    println!("the value of y is: {}", y);
    let int: i32 = 10;
    println!("the value of int is: {}", int);
    let float: f64 = 10.0;
    println!("the value of float is: {}", float);
    let bool: bool = true;
    println!("the value of bool is: {}", bool);
    let char: char = 'a';
    println!("the value of char is: {}", char);
    println!("----tuple----");
    let tuple: (i32, f64, u8) = (1, 2.0, 3);
    println!("the value of tuple is: {}", tuple.0);
    println!("the value of tuple is: {}", tuple.1);
    println!("the value of tuple is: {}", tuple.2);
    println!("----array----");
    let array: [i32; 5] = [1, 2, 3, 4, 5];
    println!("the value of array is: {}", array[0]);
    println!("the value of array is: {}", array[1]);
    println!("the value of array is: {}", array[2]);
    println!("the value of array is: {}", array[3]);
    println!("the value of array is: {}", array[4]);

    // ownership
    println!("----owernship----");
    let s1 = String::from("hello");
    let s2 = s1;// owership is moved to s2
    // println!("the value of s1 is: {}", s1);
    println!("the value of s2 is: {}", s2);

    // borrowing
    println!("----borrowing----");
    let s1 = String::from("hello");
    let s2 = &s1; // borrow using memory address
    println!("the value of s1 is: {}", s1);
    println!("the value of s2 is: {}", s2);

    println!("----function----");
    let result = add(4, 5);
    println!("the value of result is: {}", result);

    println!("----control flow----");
    let age: i32 = 18;
    if age >= 18 {
        println!("you are old enough");
    } else {
        println!("you are not old enough");
    }

    println!("----loop----");
    let mut count = 0;
    loop {
        count += 1;
        println!("count: {}", count);
        if count == 100 {
            break;
        }
    }
    while count < 100 {
        count += 1;
        println!("count: {}", count);
    }
    for x in 0..100 {
        println!("count: {}", x);
        
    }
    let arr = [1, 2, 3, 4, 5];
    for x in arr.iter() {
        println!("count: {}", x);
    }

    println!("----match----");
    let color = "green";
    let result = match color {
        "red" => "red",
        "blue" => "blue",
        "green" => "green",
        _ => "unknown",
    };
    println!("the value of result is: {}", result);

    println!("----struct----");
    let rect = Rectangle {
        width: 10,
        height: 20,
    };
    println!("the value of rect is: {}", rect.width);
    println!("the value of rect is: {}", rect.height);

    let area = calculate_area(&rect);
    println!("the value of area is: {}", area);

    println!("----pattern matching----");
    let x = 3;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("other"), // default case
    };

    println!("----option----");
    let x = Some(5);
    match x {
        Some(1) => println!("one"),
        Some(2) => println!("two"),
        Some(3) => println!("three"),
        _ => println!("other"), // default case
    };

    println!("----Error Handling----");
    println!("----result----");
    let file = File::open("hello.txt");
    match file {
        Ok(f) => println!("file opened successfully: {:?}", f),
        Err(error) => println!("Failed to open file: {}", error),
    };

    println!("----option----");
    let numbers = vec![1, 2, 3];
    match numbers.get(0) {
        Some(n) => println!("The first number is: {}", n),
        None => println!("The vector is empty"),
    };
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}
struct Rectangle {
    width: u32,
    height: u32,
}

fn calculate_area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
