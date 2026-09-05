fn main() {
	let toshiba:f64 = 450000.00 * 2.0;
	let mac:f64 = 1500000.00 * 1.0;
	let hp:f64 = 750000.00 * 3.0;
	let dell:f64 = 2850000.00 * 3.0;
	let acer:f64 = 250000.00 * 1.0;

	let sum = toshiba + mac + hp + dell + acer;
	let average = sum/5.0;

	println!("Total sales = {}", sum);
	println!("Average sales = {}", average);
}
	
	
