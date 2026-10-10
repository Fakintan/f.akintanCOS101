use std::io;

fn main() {
    println!("Welcome to my shape calculator😊");
    //the menu of the calculator
    println!("Choose a shape to calculate:");
    println!("Shapes         |     ");
    println!("1.trapezium      | Area");
    println!("2.Rhombus        | Area");
    println!("3.Parallelogram  | Area");
    println!("4.Cube           | Surface Area");
    println!("5.Cylinder       | Volume");

    let mut choice = String::new();

    println!("Enter your choice from 1-5:");

    io::stdin().read_line(&mut choice).expect("Failed to read input");
    let choice:u32 = choice.trim().parse().expect("Enter a number");

    if choice == 1{
        trapezium();
    } else if choice == 2{
        rhombus();
    } else if choice == 3{
        parallelogram();
    } else if choice == 4{
        cube();
    } else if choice == 5{
        cylinder();
    } else {
        println!("Invalid choice, enter a number from 1 to 5");
    }

}

fn trapezium() {
    println!("Enter the height:");

    let mut height = String::new();
    io::stdin().read_line(&mut height).expect("Failed to read input");
    let height:f64 = height.trim().parse().expect("Invalid number");

    println!("Enter the first base:");
    let mut base1 = String::new();
    io::stdin().read_line(&mut base1).expect("Failed to read input");
    let base1:f64 = base1.trim().parse().expect("Invalid number");

    println!("Enter the second base:");
    let mut base2 = String::new();
    io::stdin().read_line(&mut base2).expect("Failed to read input");
    let base2:f64 = base2.trim().parse().expect("Invalid number");

    let area = height / 2.0 * (base1 + base2);
    println!("The area of the trapezium is: {}", area);
    
}

fn rhombus() {
    println!("Enter the first diagonal:");

    let mut diagonal1 = String::new();
    io::stdin().read_line(&mut diagonal1).expect("Failed to read input");
    let diagonal1:f64 = diagonal1.trim().parse().expect("Invalid number");

     println!("Enter the second diagonal");
     let mut diagonal2 = String::new();
     io::stdin().read_line(&mut diagonal2).expect("Failed to read input");
     let diagonal2:f64 = diagonal2.trim().parse().expect("Invalid number");

     let area = 0.5 * diagonal1 * diagonal2;
     println!("The area of the rhombus is: {}", area);
     

 }

 fn parallelogram() {

    println!("Enter the base:");

    let mut base = String::new();
    io::stdin().read_line(&mut base).expect("Failed to read input");
    let base:f64 = base.trim().parse().expect("Invalid number");

    println!("Enter the altitude:");

    let mut altitude = String::new();
    io::stdin().read_line(&mut altitude).expect("Failed to read input");
    let altitude:f64 = altitude.trim().parse().expect("Invalid number");

    let  area = base * altitude;
    println!("The area of the Parallelogram is: {}", area);
 }

 fn cube() {

    println!("Enter the side:");

    let mut side = String::new();
    io::stdin().read_line(&mut side).expect("Failed to read input");
    let side:f64 = side.trim().parse().expect("Invalid number");

    let surface_area = 6.0 * side * side;
    println!("The suface area of the cube is: {}", surface_area);
 }

 fn cylinder() {

    println!("Enter the radius:");

    let mut radius = String::new();
    io::stdin().read_line(&mut radius).expect("Failed to read input ");
    let radius:f64 = radius.trim().parse().expect("Invalid number");

    println!("Enter the height");

    let mut height = String::new();
    io::stdin().read_line(&mut height).expect("Failed to read input");
    let height:f64 = height.trim().parse().expect("Invalid number");

    let pi = 3.14159;
    let volume = pi * radius * radius * height;

    println!("The volume of the cylinder is: {}", volume);

}



