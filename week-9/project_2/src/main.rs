use std::io;
use std::fs::OpenOptions;
use std::io::Write;
fn main() {
	let mut file = OpenOptions::new().create(true).append(true).open("PAU_SMIS.csv").expect("cannot open file");
    let mut students = Vec::new();

	let mut num = String::new();
	println!("How many student details would you like  to input");
	
	io::stdin().read_line(&mut num).expect("Failed to read input");
	let num1:usize= num.trim().parse().expect("Please input a valid digit");

	for i in 0..num1 {
		let mut name = String::new();
		println!("Enter student's name {}", i + 1);
		io::stdin().read_line(&mut name).expect("Failed to read input");

		let mut matric_no = String::new();
		println!("Enter matric number");
		io::stdin().read_line(&mut matric_no).unwrap();

		let mut dept = String::new();
		println!("Enter the department");
		io::stdin().read_line(&mut dept).unwrap();

		let mut level = String::new();
		println!("Enter the student's level");
		io::stdin().read_line(&mut level).unwrap();
		let level1:usize = level.trim().parse().unwrap();

		students.push((name.trim().to_string(), matric_no.trim().to_string(), dept.trim().to_string(), level1));
	}

	for student in students{
		println!("{}, {}, {}, {}", student.0, student.1, student.2, student.3);
	
	let record = format!("{}, {}, {}, {}\n", 
		student.0, 
		student.1, 
		student.2, 
		student.3);

	file.write_all(record.as_bytes()).unwrap();
    }

    println!("The students details has been saved successfully");

 



	

    
}
