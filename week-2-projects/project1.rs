fn main() {
	let p: f64 = 520_000_000.0;
	let r: f64 = 10.0;
	let n: f64 = 5.0;
	println!("The Principal ₦520,00,000");
	println!("The Rate is 10%");
	println!("The number of years is 5 years");
	println!("Amount= P[1+(R/100)]");
	let a = p * (1.0 + r / 100.0).powf(n);
	println!("Amount is {}", a);
	println!("Compound Interest= Amount-Principal");
	let ci = a - p;
	println!("Compound interest is {}", ci);
}