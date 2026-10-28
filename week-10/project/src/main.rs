struct Laptop {
    brand:String,
    price:u32,
    quantity:u32
}
impl Laptop {
    fn total_cost(&self)->u32 {
        self.price * self.quantity
    }
}

fn main() {

    println!("===========================================");
    println!("Welcome to my Electronics shop");
    println!("Brand      Quantity          Price each");
    println!("HP          10               N650_000");
    println!("IBM         6                N755_000");
    println!("Toshiba     10               N550_000");
    println!("Dell        4                N850_000");

    let lap1 = Laptop{
        brand:String::from("HP"),
        price:650_000,
        quantity:3
    };

    let lap2 = Laptop{
        brand:String::from("IBM"),
        price:755_000,
        quantity:3
    };

    let lap3 = Laptop {
        brand:String::from("Toshiba"),
        price:550_000,
        quantity:3
    };
    let lap4 = Laptop {
        brand:String::from("Dell"),
        price:850_000,
        quantity:3
    };

    let total1 = lap1.total_cost();
    let total2 = lap2.total_cost();
    let total3 = lap3.total_cost();
    let total4 = lap4.total_cost();

    println!("The cost for 3 {} laptop is {}", lap1.brand, total1);
    println!("The cost for 3 {} laptop is {}", lap2.brand, total2);
    println!("The cost for 3 {} laptop is {}", lap3.brand, total3);
    println!("The cost for 3 {} laptop is {}", lap4.brand, total4);

    let total = total1 + total2 + total3 + total4;

    println!("The total cost for your purchase is {}", total);

}
