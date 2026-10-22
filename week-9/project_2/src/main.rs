use std::io::Write;

fn main() {
// creating the Pau-smis file
    let mut file = std::fs::File::create("PAU-SMIS.csv").expect("create failed");
    file.write_all("                 PAU-SMIS                 \n"
        .as_bytes()).expect("write failed");
    file.write_all("Student Name, Matric. Number, Department, Level\n"
        .as_bytes()).expect("write failed");
// Storing the student details as vectors
    let students = vec![("Oluchi Mordi", "ACC10211111", "Accounting", "300"),
                        ("Adams Aliyu", "ECO10110101", "Economics", "100"),
                        ("Shania Bolade", "CSC10328828", "Computer", "200"),
                        ("Adekunle Gold", "EEE11020202", "Electrical", "200"),
                        ("Blanca Edemoh", "MEE10202001", "Mechanical", "100"),];

    for student in students {
        println!("{}, {}, {}, {}",student.0, student.1, student.2, student.3);

    let mut record = String::new();

    record.push_str(student.0);
    record.push_str(",");
    record.push_str(student.1);
    record.push_str(",");
    record.push_str(student.2);
    record.push_str(",");
    record.push_str(student.3);
    record.push_str("\n");

    file.write_all(record.as_bytes()).expect("write failed");

    }


    
}
