// Rust program to output name and age

use std::io;

fn main() {
    println!("Student Information Management System");

    // input name
    println!("Please Enter your name");
    println!("");
    let mut name = String::new();
        io::stdin()
        .read_line(&mut name)
        .expect("Failed to read input");
    println!("Your name is {}", name);

    // input age
    println!("Enter your age.");
    let mut age = String::new();
        io::stdin()
        .read_line(&mut age). expect("Failed to read input");
    let age:u8 = age. trim().parse().expect("Input not an integer");
    println!("your age is {}", age);


}
