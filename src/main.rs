fn main() {
    // simple variables
    //let x: i32 = 5;
    // println!("The value of x is: {}", x);

    // signed and unsigned integers
    // i8, i16, ...., i128
    // u8, u16, ...., u128

    // let mut x: i64 = 5;

    // for i in 0..100{
    //     x = x + i;
    // }

    // println! ("X value after for loop: {}", x);

    // primitive data types
    // strings
    // numbers (int and float)
    // booleans

    // let greeting: String = String::from("hello guys");

    // let char1: Option<char> = greeting.chars().nth(1);

    // match char1{
    //     Some(c) => println! ("{}", c),
    //     None => println!("No char found at {}", 1),
    // }

    // let num: i32 = 128;
    // if check_even(num){
    //     println!("The number {} is even!!", num);
    // }
    // else{
    //     println!("The number {} is odd!!", num);
    // }

    // Memory management
    // Why rust is better than C/C++ in terms of memory management
    // Mutability
    // Ownership
    // Heap and stack
    // Lifetimes
    // Borrowing and References
    // Rust allows fine grained control over memory usage
    // with a strong emphasis on safety and performance

    // Mutability
    // by default all variables are immutable because
    // it increases speed by avoiding race conditions
    // race conditions occur when multiple threads
    // access the same memory location simultaneously
    // and at least one of the accesses is a write
    // since all variables are immutable by default
    // there will be no race conditions
    // if we want to make a variable mutable
    // we can use the mut keyword
    // let mut x: i32 = 5;
    // TL;DR: Immutable data is inherently thread-safe

    // Stack vs Heap
    // Stack is used for static memory allocation
    // Stack: faster allocation and deallocation
    // Stack: limited size
    // Stack: stores data with known, fixed size at compile time
    // Heap is used for dynamic memory allocation
    // Heap: slower allocation and deallocation
    // Heap: larger size
    // Heap: stores data with unknown or variable size at compile time
    // Rust uses stack for primitive data types
    // Rust uses heap for complex data types like String, Vec, HashMap etc
    // for complex data types rust uses a pointer to the heap memory
    // and the pointer is stored in the stack

    // Ownership
    // Each value in Rust has a variable that’s called its owner.
    // There can only be one owner at a time.
    // When the owner goes out of scope, the value will be dropped.
    // This is how Rust manages memory without a garbage collector.
    // Stack variables are automatically cleaned up when they go out of scope
    // Heap variables are cleaned up when their owner goes out of scope
    // Example:
    // {
    //     let s1 = String::from("hello");
    //     let s2 = s1; // s1 is moved to s2
    //     // println!("{}", s1); // this will give an error because s1 is no longer valid
    //     println!("{}", s2); // this will work
    // } // s2 goes out of scope and the memory is freed        
    // Another example:
    // fn main() {
    //     let s1 = String::from("hello");
    //     take_ownership(s1);
    //     // println!("{}", s1); // this will give an error because s1 is no longer valid
    // } // s1 goes out of scope and the memory is freed
    // fn take_ownership(s: String) {
    //     println!("{}", s);
    // } // s goes out of scope and the memory is freed
    // .clone() method can be used to create a deep copy of the data

    // Borrowing and References
    // Borrowing is a way to let multiple parts of your code access data without taking ownership of it
    // This is done through references, which are pointers to the data
    // References can be either mutable or immutable
    // Mutable references allow you to change the data they point to
    // Immutable references do not allow you to change the data they point to
    // Rust enforces rules to ensure that references are always valid
    // These rules help prevent common programming errors like dangling pointers and data races
    // References are immutable by default
    // We can use & to create a reference to a variable
    // We can use &mut to create a mutable reference to a variable
    // We can have multiple immutable references to a variable
    // We can have only one mutable reference to a variable
    // We cannot have both mutable and immutable references to a variable at the same time
    // Example:
    // fn main() {
    //     let mut s = String::from("hello");
    //     let r1 = &s; // no problem
    //     let r2 = &s; // no problem
    //     println!("{} and {}", r1, r2);
    //     let r3 = &mut s; // BIG PROBLEM
    //     println!("{}", r3);
    // } // r1 and r2 go out of scope, then r3 goes out of scope and the memory is freed
    // To fix the above problem we can use a new scope
    // fn main() {
    //     let mut s = String::from("hello");
    //     {
    //         let r1 = &s; // no problem
    //         let r2 = &s; // no problem
    //         println!("{} and {}", r1, r2);
    //     } // r1 and r2 go out of scope here, so we can make a mutable reference
    //     let r3 = &mut s; // no problem
    //     println!("{}", r3);
    // } // r3 goes out of scope and the memory is freed
    
    // Structs
    // Structs are used to create custom data types
    // Structs are similar to classes in other programming languages
    // Example:
    // struct User{
    //     username: String,
    //     email: String,
    //     sign_in_count: u64,
    //     active: bool,
    // }
    // fn main() {
    //     let user1 = User{
    //         username: String::from
    //         ("karthick"),
    //         email: String::from("
    //              veerakarthick@gmail.com"),
    //         sign_in_count: 1,
    //         active: true,
    //     };
    //     println!("Username: {}", user1.username);
    // }
    // to add methods to structs we use impl keyword
    // impl User{
    //     fn display_username(&self){
    //         println!("Username: {}", self.username);
    //     }
    // }
    // fn main() {
    //     let user1 = User{
    //         username: String::from
    //         ("karthick"),
    //         email: String::from("
    //          veerakarthick@gmail.com"),
    //         sign_in_count: 1,
    //         active: true,
    //     };   
    //     user1.display_username();
    // }
    
    // Enums
    // Enums are used to create custom data types
    // Enums are similar to structs but they can have multiple variants
    // Example:
    // enum IpAddrKind{
    //     V4,
    //     V6,
    // }
    // struct IpAddr{
    //     kind: IpAddrKind,
    //     address: String,
    // }
    // fn main() {
    //     let home = IpAddr{
    //         kind: IpAddrKind::V4,
    //         address: String::from("
    //      address"),
    //     };
    //     let loopback = IpAddr{
    //         kind: IpAddrKind::V6,
    //         address: String::from("::1"),
    //     };
    // }
    // Enums can also have data associated with each variant
    // Example:
    // enum Shape{
    //     Circle(f64), // radius
    //     Rectangle(f64, f64), // width, height
    // }
    // fn main() {
    //     let circle = Shape::Circle(5.0);
    //     let rectangle = Shape::Rectangle(10.0, 20.0);
    // }
    // to use the data associated with each variant
    // we can use match statement
    // fn area(shape: Shape) -> f64{
    //     match shape{
    //         Shape::Circle(radius) => 3.14 * radius * radius,
    //         Shape::Rectangle(width, height) => width * height,
    //     }
    // }

    // Pattern Matching
    // Pattern matching is a powerful feature in Rust
    // It allows us to match values against patterns
    // and execute code based on the matched pattern
    // Example:
    // enum Message{
    //     Quit,
    //     Move{x: i32, y: i32},
    //     Write(String),
    //     ChangeColor(i32, i32, i32),
    // }
    // fn main() {
    //     let msg = Message::Move{x: 10, y: 20};
    //     match msg{
    //         Message::Quit => println!("Quit"),
    //         Message::Move{x, y} => println!("Move to ({}, {})",
    //         x, y),
    //         Message::Write(text) => println!("Text message: {}", text),
    //         Message::ChangeColor(r, g, b) => println!("Change color to ({}, {}, {})",
    //         r, g, b),
    //     }
    // }    
    
    // Option Enum
    // Option enum is used to represent a value that can be either
    // something or nothing
    // Example:
    // fn find_first_a(s: String) -> Option<i32> {
    //     for (i, c) in s.chars().enumerate() {
    //         if c == 'a' {
    //             return Some(i as i32);
    //         }
    //     }
    //     None
    // }
    // fn main() {
    //     let s = String::from("hello");
    //     match find_first_a(s) {
    //         Some(index) => println!("Found 'a' at index: {}", index),
    //         None => println!("'a' not found"),
    //     }
    // }

    // Error Handling
    // Rust has a strong emphasis on error handling
    // Rust has two types of errors: recoverable and unrecoverable
    // Recoverable errors are handled using the Result enum
    // Unrecoverable errors are handled using the panic! macro
    // Example of recoverable error:
    // use std::fs;
    // enum Result<A, B>{
    //      oK(A),
    //      Err(B),
    // }
    // fn main() {
    //     let file_path = "hello.txt";
    //     let contents: Result<String, Error> = fs::read_to_string(file_path);
    // match res{
    //    Ok(s) => println!("File contents: {}", s),
    //    Err(e) => println!("Error reading file: {}", e),
    // }
    // }
    


}

// pub fn check_even(x: i32) -> bool {
//     return x%2 == 0;
// }