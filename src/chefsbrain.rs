use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let mut iter = input.trim().split_whitespace();
    let x: i32 = iter.next().expect("No value found").parse().expect("Failed to parse integer");
    let y: i32 = iter.next().expect("No value found").parse().expect("Failed to parse integer");

    // Your code here
    if x < y {
        println!("YES");
    }
    else{
        println!("NO");
    }
}
