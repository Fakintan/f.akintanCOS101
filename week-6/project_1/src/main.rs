use std::io;
fn main() {
    println!("Welcome to Faith's Restaurant");
    println!("What would you like to order today?😊\n");

    println!("--------------------------------------------");
    println!("              RESTAURANT MENU            ");
    println!("--------------------------------------------");

    println!("P - Poundo Yam and Edikaikong soup: N3_200");
    println!("F - Fried Rice and Chicken: N3_000");
    println!("A - Amala and Ewedu Soup: N2_500");
    println!("E - Eba and Egusi Soup: N2_000");
    println!("W - White Rice and Stew: N2_500");
    println!("--------------------------------------------");

    println!("Note: There's a 5% discount when your purchase is greater than N10_000");

    println!("\nEnter the first letter of your desired food (P,F,A,E,W):");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read input");

    let price: u32;

    if choice.trim().to_uppercase() == "P" {
        price = 3200;
    }
    else if choice.trim().to_uppercase() == "F" {
        price = 3000;
    }
    else if choice.trim().to_uppercase() == "A" {
        price = 2500;
    }
    else if choice.trim().to_uppercase() == "E" {
        price = 2000;
    }
    else if choice.trim().to_uppercase() == "w" {
        price = 2500;
    }
    else {
        println!("Not a valid food choice");
        return;
    }

    println!("Enter the quantity of food you would like to purchase:");
    let mut quantity = String::new();

    io::stdin().read_line(&mut quantity).expect("Failed to read input");

    let quantity:u32 = quantity.trim().parse().expect("Enter a number");

    let total = price * quantity;

    println!("Total amount: N{}",total);

    if total > 10_000{
        let discount = total * 5 / 100;
        let amount = total - discount;

        println!("Discount: N{}", discount);
        println!("Amount to pay: N{}", amount);
    }
    else {
        println!("Amount to pay: N{}", total);
    }
    println!("Thank you for patronizing Faith's Restaurant, Do well to enjoy your meal");
    println!("We hope to see you next time");
}
