use std::io::Write;

fn main() {
    let mut file = std::fs::File::create("The EFCC Merge.csv").expect("Create failed");
    file.write_all("        The EFCC Merge      \n"
        .as_bytes()).expect("write failed");
    file.write_all("Commisioner, Ministry, Zone\n"
        .as_bytes()).expect("write failed");

    let commissioners = vec!["Aigbogun Alamba Daudu", "Murtala Afeez Bendu", "Okorocha Calistus Ogbona", "Adewale Jimoh Akanbi", "Osazuwa Faith Etieye"];
    let ministry = vec!["Internal Affairs", "Justice", "Defense", "Power & Steel", "Petroleum"];
    let zone = vec!["South West", "North East", "South South", "South West", "South East"];

    for i in 0..commissioners.len() {
        let mut dataset = String::new();

        dataset.push_str(commissioners[i]);
        dataset.push_str(",");
        dataset.push_str(ministry[i]);
        dataset.push_str(",");
        dataset.push_str(zone[i]);
        dataset.push_str("\n");

    file.write_all(dataset.as_bytes()).expect("write failed")
    }
    println!("The EFCC Merge file has been created successfully");
    


    
}
