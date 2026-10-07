use std::io;

fn main() {
    println!("Welcome to Faith's Restaurant");
    println!("What would you like to order today?"); 
    println!();

    println!("             RESTAURANT MENU😊              ");
    println!("=============================================");
    println!("P - |Poundo yam and edikaikong soup:   |3_200");
    println!("F - |Fried Rice and Chicken:           |3_000");
    println!("A - |Amala and Ewedu Soup:             |2_500");
    println!("E - |Eba and Egusi Soup:               |2_000");
    println!("W - |White Rice and Stwe:              |2_500");
    println!("=============================================");
    println!();
    println!("Note: There's a 5% discount when your total order is greater than N10_000");

    println!("Enter the first letter of your desired food(P, F, A, E, W):");
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read input");

    let price:u32;

    if choice.trim().to_uppercase() == "P"{
        price = 3200;
    }
    else if choice.trim().to_uppercase() == "F"{
        price = 3000;
    }
    else if choice.trim().to_uppercase() == "A"{
        price = 2500;
    }
    else if choice.trim().to_uppercase() == "E"{
        price = 2000;
    }
    else if choice.trim().to_uppercase() == "W"{
        price = 2500;
    }
    else {
        println!("Not a valid input");
        return;
    }
    println!("Enter the quantity of food you would like to order");
    let mut quantity = String::new();
    io::stdin().read_line(&mut quantity).expect("Failed to read input");

    let quantity:u32 = quantity.trim().parse().expect("Please enter a number");

    let total = price * quantity;
    println!("Total amount: N{}", total);

    if total > 10_000 {
        let discount = total * 5 / 100;
        let amount = total - discount;

        println!("You have a discount of: {}", discount);
        println!("Total amount to pay: {}", amount);

    } else {
        println!("Total amount to pay: {}", total);
    }

    println!("Thank you for patronizing Faith's Restaurant");
    println!("Do well to come back next time for more sumptous meal");


}



