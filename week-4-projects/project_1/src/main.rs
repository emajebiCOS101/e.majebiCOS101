    // Rust program to calculate the roots of values a,b,c

    use std::io;

    fn main(){


    let mut input1 = String::new();
    println!("Enter the value of a:");
    io::stdin().read_line(&mut input1).expect("Failed to read input");

    let mut input2 = String::new();
    println!("Enter value of b:");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    
    let mut input3 = String::new();
    println!("Enter value of c:");
    io::stdin().read_line(&mut input3).expect("Failed to read input");

    let a: f32 = input1.trim().parse().expect("Enter a number");
    let b: f32 = input2.trim().parse().expect("Enter a number");
    let c: f32 = input3.trim().parse().expect("Enter a number");

    if a == 0.0 {
        println!("This is not a quadratic equation (a cannot be 0).");
    } else {
        let discriminant = b * b - 4.0 * a * c;

        println!("Discriminant = {}", discriminant);

        if discriminant > 0.0 {
            let root1 = (-b + discriminant.sqrt()) / (2.0 * a);
            let root2 = (-b - discriminant.sqrt()) / (2.0 * a);
            println!("There are two distinct real roots:");
            println!("Root 1 = {}", root1);
            println!("Root 2 = {}", root2);
        } else if discriminant == 0.0 {
            let root = -b / (2.0 * a);
            println!("There is one real root:");
            println!("Root = {}", root);
        } else {
            // no real roots
            println!("There are no real roots (discriminant is negative)."); 
        }
    }
}