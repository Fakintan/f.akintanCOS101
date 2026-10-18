use std::io;
fn main() {

    println!("Welcome to The EY Interview");
    let mut interviewee: Vec<(String, u32)> = Vec::new();

    println!("How many candidates to do you want to input?:");

    let mut num = String::new();
    io::stdin().read_line(&mut num).expect("Failed to read input");
    let total_candidate:usize = num.trim().parse().expect("Please input a valid number");

    for i in 1..=total_candidate {
        println!("\n Inputting the details for candidate {}", i);
    

    let mut name = String::new();

    println!("Input interviewee's name");
    io::stdin().read_line(&mut name).expect("Please enter a valid name");

    println!("Enter years of programmming experiece: ");

    let mut years = String::new();
    io::stdin().read_line(&mut years).expect("Please enter a valid number ");
    let years:u32 = years.trim().parse().expect("Invalid input");
    

    interviewee.push((name.trim().to_string(), years)); }

    let mut highest_index = 0;

    for i in 1..interviewee.len() {
        if interviewee[i].1 > interviewee[highest_index].1 {
            highest_index = i;
        }
    }
    println!("The most experienced is: {}", interviewee[highest_index].0);
    println!("The years of experiece: {}", interviewee[highest_index].1);



}
