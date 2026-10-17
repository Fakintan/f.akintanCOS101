use std::io;

fn main() {
    let levels = vec!["APS 1-2", "APS 3-5", "APS 5-8", "EL1 8-10", "EL2 10-13", "SES"];

    let public_servant = vec!["APS 1-2", "APS 3-5", "APS 5-8", "EL1 8-10", "EL2 10-13", "SES"];

    let office_admin = vec![
        "Intern",
        "Administrator",
        "Senior Administrator",
        "Office Manager",
        "Director",
        "CEO"
];

    let academic = vec![
        " ",
        "Research Assistant",
        "PhD Candidate",
        "Past-Doc Researcher",
        "Senior Lecturer",
        "Dean"];

    let lawyer = vec![
        "Paralegal",
        "Junior Assistant",
        "Associate",
        "Senior Associate 1-2",
        "Senior Associate 3-4",
        "Partner"];
    let teacher = vec![
        "Placement",
        "Classroom Teacher",
        "Senior Teacher",
        "Leading Teacher",
        "Deputy Principal",
        "Principal"];


    let mut profession = String::new();

    println!("The APS Level Checker");
    print!("Choose your profession:");
    println!("1. Public Servant");
    println!("2. Office Admin");
    println!("3. Academic");
    println!("4. Lawyer");
    println!("5. Teacher");
    println!("6. I don't know");
    println!("Input your profession from 1-6"); 

    io::stdin().read_line(&mut profession).expect("Failed to read input");
    let profession:u32 = profession.trim().parse().expect("Invalid input");

    let mut experience = String::new();
    println!("Input your years of experience");
    io::stdin().read_line(&mut experience).expect("Failed to read input");
    let years:u32 = experience.trim().parse().expect("Please enter a valid");

    let index;

    if years <= 2 {
        index = 0;
    }
    else if years <= 5 {
        index = 1;

    }
    else if years <= 8 {
        index = 2;
    }
    else if years <= 10{
        index = 3;
    }
    else if years <= 15{
        index = 4;
    }
    else {
        index = 5;
    }
    

    if profession < 1 || profession > 6 {
        println!("The profession choice is not valid");
    }
    else {
        println!("Your level is {}", levels[index]);
    

      if profession == 1{
        println!("Your position is: {}", public_servant[index]);
      }
      else if profession == 2{
        println!("Your position is: {}", office_admin[index]);
      }
      else if profession == 3 {
        println!("Your position is: {}", academic[index]);
      }
      else if profession == 4 {
        println!("Your position is: {}", lawyer[index]);
      }
      else if profession == 5 {
        println!("Your position is:{}", teacher[index]);
      } else {
        println!("Your profession was not specified.");
      }
    }
} 

