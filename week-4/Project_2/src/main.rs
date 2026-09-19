use std::io;

fn main() {
    let mut experience = String::new();
    let mut age = String::new();

    println!("Is the employee experienced? (Yes / No:");
    io::stdin().read_line(&mut experience).expect("Not a Valid String");

    println!("Input Employee's age: ");
    io::stdin().read_line(&mut age).expect("Not a valid String");
    let age:i8 = age.trim().parse().expect("Not a valid Number");


    if experience.trim() == "Yes" && age >= 40 {
        println!("Annual incentive: #1_560_000");
    }
    else if experience.trim() == "Yes" && age >= 30 && age <= 38 {
        println!("Annual incentive: #1_480_000");

    }
    else if experience.trim() == "Yes" && age < 28 {
        println!("Annual incentive: #1_300_000");

    }
    else if experience.trim() == "No" {
        println!("Annual incentive: #100_000");
    } 
    else {
        println!("Not a valid input");
    }


   


    







}
