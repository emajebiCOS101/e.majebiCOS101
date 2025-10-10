fn main() {
	let toshiba: f64 = 450_000.00;
	let mac: f64 = 1_500_000.00;
	let hp: f64 = 750_000.00;
	let dell: f64 = 2_850_000.00;
	let acer: f64 = 250_000.00;
	let s = toshiba + mac + hp + dell + acer;
	let a = s / 2.0;
	println!("The sales of Toshiba is ₦450,000");
	println!("The sales of Mac is ₦150,000,000");
	println!("The sales of Hp is ₦750,000");
	println!("The sales of Dell is ₦2,850,000");
	println!("The sales of Acer is ₦250,000");
	println!("The sum of the sales is {}", s);
	println!("The average of the sales is {}", a);
}