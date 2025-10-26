use std::io;

fn main() {
    let mut experience_input = String::new();
    let mut age_input = String::new();

    println!("Is the employee experienced? (yes/no):");
    io::stdin().read_line(&mut experience_input).expect("Failed to read input");

    println!("Enter age of the employee:");
    io::stdin().read_line(&mut age_input).expect("Failed to read input");

    let experience = experience_input.trim().to_lowercase();
    let age: i32 = age_input.trim().parse().expect("Please enter a valid number");

    if experience == "yes" {
        if age >= 40 {
            println!("The annual incentive is ₦1,560,000.");
        } else if age >= 30 && age < 40 {
            println!("The annual incentive is ₦1,480,000.");
        } else if age < 28 {
            println!("The annual incentive is ₦1,300,000 per month.");
        }
    } else if experience == "no" {
        println!("The annual incentive is ₦100,000.");
    } else {
        println!("Invalid input for experience. Please type 'yes' or 'no'.");
    }
}
