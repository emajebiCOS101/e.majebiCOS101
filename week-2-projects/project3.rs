fn main() {
	let p: f64 = 510_000.0;
	let r: f64 = 5.0;
	let n: f64 = 3.0;
	let a = p * (1.0 - r / 100.0).powf(n);
	println!("The Principal is ₦510,000");
	println!("The rate of depreciation is 5%");
	println!("The number of years is 3 years");
	println!("Amount= P[1-(R/100)]^n", );
	println!("Amount is {}", a);
	println!("Deprciation = Amount-Principal");
	let d = a - p;
	println!("Deprciation is {}", d);
}