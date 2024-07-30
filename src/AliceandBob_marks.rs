use std::io;
fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let mut iter = input.trim().split_whitespace();
    let alice_marks:u32 = iter.next().expect("Value not found").parse().expect("failed to parse");
    let bob_marks:u32 = iter.next().expect("Value not found").parse().expect("Failed to parse");

    if alice_marks>=2*bob_marks {
        println!("YES");
    }
    else{
        println!("NO");
    }
}