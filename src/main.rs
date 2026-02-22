// Libraries for the file reader
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// For reading CLI args
use std::env;

// Efficient implementation from Rust docs
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// Generates all possible words
fn generate_dictionary() -> Vec<String> {
    let possible_words: Vec<String> = vec![];

    possible_words
}



fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <dic_path> <aff_path>", args[0]);
        return;
    }
    let dic_path: &str = &args[1];
    let aff_path: &str = &args[2];

    // Print out .dic file contents
    if let Ok(lines) = read_lines(dic_path) {
        for line in lines.flatten() {
            println!("{}", line);
        }
    }

    
    println!("Dict path:{}, Aff path: {}", dic_path, aff_path);
}
