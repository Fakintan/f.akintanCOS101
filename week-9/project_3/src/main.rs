use std::io::Write;

use std::io::Read;

fn main() {
    //let mut records = Vec::new();
    let mut file1 = std::fs::File::open("Commissioner.csv").expect("open failed");
    let mut commissioner = String::new();
    file1.read_to_string(&mut commissioner).expect("failed");
    let commissioner = commissioner.trim().split("\r\n");
    let mut comms: Vec<&str> = Vec::new();

    let mut file2 = std::fs::File::open("Ministry.csv").unwrap();
    let mut ministry = String::new();
    file2.read_to_string(&mut ministry).expect("failed");
    let ministry = ministry.trim().split("\r\n");
    let mut mins: Vec<&str> = Vec::new();

    let mut file3 = std::fs::File::open("Zone.csv").unwrap();
    let mut zone = String::new();
    file3.read_to_string(&mut zone).expect("failed");
    let zone = zone.trim().split("\r\n");
    let mut zon: Vec<&str> = Vec::new();

    let mut file4 = std::fs::File::create("The EFCC Merge.csv").expect("create failed");
    

    for a in commissioner{
        comms.push(a);
    }

    for b in ministry{
        mins.push(b);
    }

    for c in zone{
        zon.push(c);
    }
    println!("😊😊😊😊😊😊😊😊😊😊😊😊😊 Your files have been merged successfully!");
     
    for i in 0..6{
        file4.write_all(format!("{}, {}, {}\n", comms[i], mins[i], zon[i]).as_bytes()).unwrap();
    }
    
    
   
   




  
}