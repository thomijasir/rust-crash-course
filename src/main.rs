#![allow(dead_code)]
#![allow(unused_variables)]
// VIDEO TUTORIAL https://www.youtube.com/watch?v=rQ_J9WH6CGk
fn primitive_data_type() {
    // rust has signed (+ and-) and use unsigned (-) integer (only+) types of different sizes
    // i8, i16, i32, i64, i128: signed integers
    // u8, u16, u32, u64, u128: unsigned integers
    // When to use which type
    // - Use unsigned when you know the value will never be negative (e.g., array indices, counts).
    // - Use signed when the value could be negative or when you’re doing math that could result in negative numbers.
    // - Be cautious when mixing signed and unsigned types in operations to avoid unexpected results.

    let x: i32 = -5;
    let y: u64 = 10;
    println!("Signed Integer: {}", x);
    println!("Unsigned Integer: {}", y);
}

fn compound_data_type() {
    // compound data types are made up of primitive data types
    // arrays, tuples, structs, enums and strings (slice string)
    // arrays
    let numbers: [i32; 3] = [1, 2, 3];
    println!("Numbers: {:?}", numbers);
    // tuples
    let pair: (i32, f64) = (1, 2.0);
    println!("Pair: {:?}", pair);

    // array of strings
    let names: [&str; 3] = ["Alice", "Bob", "Charlie"];
    println!("Names: {:?}", names);

    // array string slice
    let animal_slice: &[&str] = &["cat", "dog", "bird"];
    println!("Animal slice: {:?}", animal_slice);

    // If you need a dynamic size, use a vector instead:
    let mut dynamic_names: Vec<&str> = vec!["Alice", "Bob", "Charlie"];
    dynamic_names.push("David");
    println!("Dynamic Names: {:?}", dynamic_names);

    // In general,
    // if you need a fixed-size collection known at compile time, use an array.
    // If you need more flexibility or are working with a portion of a collection, use a slice.
    // If you need a growable collection, consider using a Vec<&str> instead.

    // structs
    // #[derive(Debug)]
    // struct Person {
    //     name: String,
    //     age: u8,
    // }
    // let person = Person {
    //     name: "Alice".to_string(),
    //     age: 25,
    // };
}

// Functions with return type
fn addition(a: u32, b: u32) -> u32 {
    println!("a: {}, b: {}", a, b);
    a + b
}

fn mutable_ref() {
    let mut x = 5;
    let y = &mut x;
    *y = 10;
    println!("x: {}", x);
}

struct BankAccount {
    owner: String,
    balance: f64,
}

impl BankAccount {
    fn withdraw(&mut self, amount: f64) {
        println!("Withdrawing {} from account", amount);
        self.balance -= amount;
    }

    fn check_balance(&self) {
        println!(
            "account owned by {} has balance {}",
            self.owner, self.balance
        );
    }
}

fn main_bank_account() {
    let mut account = BankAccount {
        owner: "Alice".to_string(),
        balance: 100.0,
    };

    account.withdraw(50.0);
    account.check_balance();
}

fn mutable_variable() {
    // if you don't want to change the value of a variable, use let
    // use mut keyword to change the value of a variable
    let mut x: i32 = 5;
    println!("x: {}", x);
    x = 10;
    println!("x: {}", x);
}

fn constant_and_variable() {
    /*
      the important difference between a constant and a variable once const defined
      it can use in any place, including function, but it can not be changed
      const is a keyword that allows you to declare a constant value
    */
    const PI: f64 = 3.14159;
    println!("PI: {}", PI);

    // let is a keyword that allows you to declare a variable
    let x: i32 = 5;
    println!("x: {}", x);

    // let is immutable
    // let y: i32 = 10;
    // println!("y: {}", y);
    // y = 20;
    // println!("y: {}", y);
}

fn shadowing_concept() {
    // shadowing concept is when you have a variable with the same name and re use it
    let x = 5;
    let x = x + 10;
    println!("first x: {}", x);
    {
        let x = x * 2;
        println!("second x: {}", x);
    }
    // only working with in tge scope
    println!("final x: {}", x);
}

fn conditional_statement() {
    let x = 5;
    let y = 10;

    if x < y {
        println!("x is less than y");
    } else if x > y {
        println!("x is greater than y");
    } else {
        println!("x is equal to y");
    }

    println!("x is {}", x);

    let gender: String = String::from("MALE");
    const GENDER: &str = "MALE";
    let age = 25;

    if GENDER == "MALE" && age > 18 {
        println!("You can drive a car");
    }
}

fn loop_statement() {
    // 1. Loop
    // Loop normally is used to repeat a block unconditionally normally used for retrying
    let mut x = 0;
    loop {
        println!("x: {}", x);
        x += 1;
        if x == 5 {
            break;
        }
    }
    // 2. While
    // While loop is used to repeat a block while a condition is true
    let mut x = 0;
    while x < 5 {
        println!("y: {}", x);
        x += 1;
    }
    // 3. For
    // For loop is used to repeat a block for a specific number of times
    for i in 0..5 {
        println!("z: {}", i);
    }
}

#[derive(Debug)]
struct GeoLocation(f64, f64);

fn object_statement() {
    // Basic Structure
    struct Person {
        name: String,
        age: u8,
    }

    let person = Person {
        name: String::from("Alice"),
        age: 25,
    };

    println!("Person name is: {}", person.name);
    println!("Person age is: {}", person.age);

    // Destructuring in rust
    let Person { name, age } = person;
    println!("DESTRUCTURE)");
    println!("Person name is: {}", name);
    println!("Person age is: {}", age);

    let person2 = Person {
        name: String::from("Bob"),
        ..person
    };
    println!("Person2 name is: {}", person2.name);
    println!("Person2 age is: {}", person2.age);

    // Tuple Struct
    let geo: GeoLocation = GeoLocation(1.0, 2.0);
    println!("GeoLocation: {:?}", geo);
}

// enum Statement
#[derive(Debug)]
enum Color {
    Red,
    Green,
    Blue,
}

fn enum_statement() {
    let color = Color::Red;
    println!("Color: {:?}", color);

    // let color = match color {
    //     Color::Red => println!("Red"),
    //     Color::Green => println!("Green"),
    //     Color::Blue => println!("Blue"),
    // };
    // println!("Color: {:?}", color);
}

// Defining generic option type
enum Option<T> {
    Some(T), // represents Some value
    None,    // represents no value
}
// Another Sample
// enum Result<T, E> {
//     Ok(T),  // represents Ok value
//     Err(E), // represents Err value
// }
// fn sample_divide(numerator: f64, denominator: f64) -> Result<f64, String> {
//     if denominator == 0.0 {
//         Err(String::from("Cannot divide by zero"));
//     } else {
//         Ok(numerator / denominator)
//     }
// }

fn error_handling() {}

fn main() {
    // primitive_data_type();
    // compound_data_type();
    // let test: () = add(1, 2);
    // println!("test: {}", test);
    // let result = addition(1, 2);
    // println!("result: {}", result);
    // mutable_ref();
    // main_bank_account();
    // mutable_variable();
    // constant_and_variable();
    // shadowing_concept();
    // conditional_statement();
    // loop_statement();
    // object_statement();
    // enum_statement();
}
