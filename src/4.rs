use std::fs;

fn main() {
    let dir = "hello";

    match fs::create_dir(dir) {
        Ok(()) => println!("Directory '{}' has been created", dir),
        Err(err) => println!("Failure: {}", err),
    }
}
