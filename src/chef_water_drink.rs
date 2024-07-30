use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed to read");
    let num_test_cases:u32 = input.trim().parse().expect("Failed to parse");
    for _ in 0..num_test_cases {
        input.clear();
        // let mut input = String::new();
        io::stdin().read_line(&mut input).expect("failed to read");
        let test_case:u32 = input.trim().parse().expect("Failed to parse");
        if test_case>=2000{
            println!("YES");
        }
        else{
            println!("NO");
        }
    }
}