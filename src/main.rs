// Libraries for the file reader
use std::fs::File;
use std::io::{self, BufRead, Read};
use std::path::Path;

// For reading CLI args
use std::env;

// For loading .dic and .aff files
use std::collections::HashMap;

// For parsing .aff files
use regex;

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
    pub fn new(dict_path: &str) -> Self {
        Self { 
            hashmap: Self::load(dict_path)
        }
    }
    fn load(dictpath: &str) -> HashMap<String, String> {
        let mut dictionary: HashMap<String, String> = HashMap::new();

        if let Ok(lines) = read_lines(dictpath) {
            for line in lines.flatten() {
                let record: Vec<&str> = line.split("/").collect();
                if (record.len() == 2) {
                dictionary.insert(record[0].to_string(), record[1].to_string());
                }
            }
        }
        dictionary
    }
    pub fn get_rules(&self, word: &str) -> Vec<char> {
        let rules: Vec<char> = self.hashmap.get(word).unwrap().chars().collect();

        rules
    }

}

struct Rules {
    kind: String,
    flag: char,
    can_be_combined: bool,


}



fn apply_rule(word: &str) {

}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <dic_path> <aff_path>", args[0]);
        return;
    }
    let dic_path: &str = &args[1];
    let aff_path: &str = &args[2];
    println!("Dict path:{}, Aff path: {}", dic_path, aff_path);

    let dictionary = Dictionary::new(dic_path);

let mut term = String::new();
    loop {
        term.clear();
        println!("-----------------------------------");
        println!("Wpisz słowo, które chcesz wyszukać:");
        io::stdin().read_line(&mut term).expect("Błąd w odczytywaniu słowa");
        let term = term.trim();

        println!("Słowo: {}, Zasady: {:?}", term, dictionary.get_rules(&term))
    }

}
