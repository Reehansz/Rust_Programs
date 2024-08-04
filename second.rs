use std::fs::File;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    let file_path = "me.csv";
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let lines: Vec<&str> = contents.trim().split('\n').collect();
    for line in lines {
        let cells: Vec<&str> = line.split(',').collect();
        for cell in cells {
            print!("{}", cell); // Print each cell with fixed width for alignment
        }
        println!(); // Print a newline at the end of each row
    }

    Ok(())
}
