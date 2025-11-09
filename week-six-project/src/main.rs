use std::io;

fn main() {
    loop {
    println!("Menu                                 Price(₦)
           P =  Poundo Yam / Edinkaiko Soup      ₦3,200/n
           F =  Fried Rice & Chicken             ₦3,000/n
           A =  Amala & Ewedu Soup               ₦2,500/n
           E =  Eba & Egusi Soup                 ₦2,000/n 
           W =  White Rice & Stew                ₦2,500/n");

    let mut input1 = String::new();
    let mut input2 = String::new();
    println!("What do you want to order?");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let food:String = input1.trim().to_uppercase();

    println!("What quantity do you want to order?");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let quantity:f32 = input2.trim().parse().expect("Not a valid number");

    let p:f32 = 3200.0;
    let f:f32 = 3000.0;
    let a:f32 = 2500.0;
    let e:f32 = 2000.0;
    let w:f32 = 2500.0;

    let total1:f32 = p * quantity;
    let total2:f32 = f * quantity;
    let total3:f32 = a * quantity;
    let total4:f32 = e * quantity;
    let total5:f32 = w * quantity;

    let finalprice1:f32 = 0.5 * total1;
    let finalprice2:f32 = 0.5 * total2;
    let finalprice3:f32 = 0.5 * total3;
    let finalprice4:f32 = 0.5 * total4;
    let finalprice5:f32 = 0.5 * total5;

    if food == "P" && total1 > 10000.0 {
        println!("You've received a 5% discount");
        println!("Your total is {}",finalprice1 );
    } else if food == "P" && total1 <= 10000.0 {
        println!("Your total is {}",total1 );
    }

    if food == "F" && total2 > 10000.0 {
        println!("You've received a 5% discount");
        println!("Your total is {}",finalprice2 );
    } else if food == "F" && total2 <= 10000.0 {
        println!("Your total is {}",total2 );
    }

    if food == "A" && total3 > 10000.0 {
        println!("You've received a 5% discount");
        println!("Your total is {}",finalprice3 );
    } else if food == "A" && total3 <= 10000.0 {
        println!("Your total is {}",total3 );
    }
    if food == "E" && total4 > 10000.0 {
        println!("You've received a 5% discount");
        println!("Your total is {}",finalprice4 );
    } else if food == "E" && total4 <= 10000.0 {
        println!("Your total is {}",total4 );
    }
    if food == "W" && total5 > 10000.0 {
        println!("You've received a 5% discount");
        println!("Your total is {}",finalprice5 );
    } else if food == "W" && total5 <= 10000.0 {
        println!("Your total is {}",total5);
    }
    println!("Do you want to continue? (Y/N)");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Not a valid string");
    let choice:String = choice.trim().to_uppercase();

    
    if choice == "N" {
        println!("Thank you for your order!");
        break;
    }
}
}






