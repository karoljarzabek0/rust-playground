// Libraries for the file reader
use std::fs::{File, read};
use std::hash::Hash;
use std::io::{self, BufRead, Read};
use std::path::Path;

// For reading CLI args
use std::env::{self, current_exe};

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


#[derive(Debug)]
struct AffFile {
    groups: HashMap<char, RuleGroup>
}

#[derive(Debug)]
struct RuleGroup {
    kind: String,
    flag: char,
    can_be_combined: bool,
    rules: Vec<Rule>
}

#[derive(Debug)]
struct Rule {
    remove: String,
    add: String,
    condition: String
}

impl AffFile {
    pub fn new(aff_path: &str) -> Self {
        Self { 
            groups: Self::load(aff_path)
        }
    }
    fn load(aff_path: &str) -> HashMap<char, RuleGroup> {
        let mut rule_groups: HashMap<char, RuleGroup> = HashMap::new();

        if let Ok(lines) = read_lines(aff_path) {
            let mut current_flag_char: char = ' ';

            for line in lines.flatten() {
                let mut record: Vec<&str> = line.split(" ").collect();
                record.retain(|word| word.len() >= 1);

                if record.len() > 2 && (record[0] == "PFX" || record[0] == "SFX") {
                    if record[2] == "Y" || record[2] == "N" {
                        // Case when whe encouter the Rules header

                        current_flag_char = record[1].chars().next().unwrap();

                        rule_groups.entry(current_flag_char).or_insert(RuleGroup { 
                            kind: record[0].to_string(),
                            flag: record[1].to_string().chars().next().unwrap(),
                            can_be_combined: record[2] == "Y",
                            rules: vec![] }
                        );
                    } else {
                        // Case when we encounter the individual Rule case
                        match rule_groups.get_mut(&current_flag_char) {
                            Some(rule_group) => rule_group.rules.push(Rule {
                                remove: record[2].to_string(),
                                add: record[3].to_string(),
                                condition: record[4].to_string()
                            }),
                            None => panic!("Tried adding the rule out of order, sth is definetly wrong")
                        }
                    }
                } else {
                    current_flag_char = ' ';
                }

                println!("{:?}, group: {:?}", record, current_flag_char);

                }
            println!("{:?}", rule_groups);
            
            }

        rule_groups
        }
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

    let aff_file = AffFile::new(aff_path);

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
