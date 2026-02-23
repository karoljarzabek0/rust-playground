// Libraries for the file reader
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// For reading CLI args
use std::env;

// For loading .dic and .aff files
use std::collections::HashMap;

// Efficient implementation from Rust docs
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

struct Dictionary {
    hashmap: HashMap<String, String>
}

impl Dictionary {
    pub fn new() -> Self {
        Self { hashmap: HashMap::new() }
    }
    pub fn insert(&mut self, key: String, value: String) {
        self.hashmap.insert(key, value);
    }
    pub fn get_rules(&self, word: &str) -> Vec<String> {
        let mut rules: Vec<String> = vec![];

        let rule = self.hashmap.get(word).unwrap();

        rules
    }
}

fn load_dict(filename: &str) -> HashMap<String, String> {
    let mut dictionary: HashMap<String, String> = HashMap::new();

    if let Ok(lines) = read_lines(filename) {
        for line in lines.flatten() {
            let record: Vec<&str> = line.split("/").collect();
            if (record.len() == 2) {
            dictionary.insert(record[0].to_string(), record[1].to_string());
            }
        }
    }
    dictionary
}


fn apply_rule(base_word: &str, rule: &str) -> Vec<String> {
    let generated_words: Vec<String> = vec![];


    generated_words
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
            let record: Vec<&str> = line.split("/").collect();
            if (record.len() == 2) {
            println!("{}, rule: {}", record[0], record[1]);
            }
        }
    }

    
    println!("Dict path:{}, Aff path: {}", dic_path, aff_path);
}
