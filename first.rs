use std::env;


fn main() {
    let args: Vec<String> = env::args().collect();

    // Parse the arguments as integers
    let num1: i32 = args[0].parse().expect("Please provide a valid number");
    let num2: i32 = args[1].parse().expect("Please provide a valid number");
    let num3: i32 = args[2].parse().expect("Please provide a valid number");
    let num4: i32 = args[3].parse().expect("Please provide a valid number");
    
    let sum = num1 + num2 + num3 + num4;

    println!("The sum of {} + {} + {} + {} is {}", num1, num2, num3, num4, sum);
}
