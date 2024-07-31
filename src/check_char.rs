fn check_case(s: &str) -> &str {
    let chars: Vec<char> = s.chars().collect();

    if chars.iter().all(|c| c.is_uppercase()) {
        "Uppercase characters!"
    } else if chars.iter().all(|c| c.is_lowercase()) {
        "lowercase characters!"
    } else {
        "Mixed case characters!"
    }
}

fn main() {
    let test1 = "HELLO";
    let test2 = "hello";
    let test3 = "Hello";

    println!("{}", check_case(test1)); 
    println!("{}", check_case(test2)); 
    println!("{}", check_case(test3)); 
}