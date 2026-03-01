use std::fmt::format;
// Libraries for the file reader
use std::fs::{File, read};
use std::hash::Hash;
use std::io::{self, BufRead, BufWriter, Read, Write};
use std::path::Path;

// For reading CLI args
use std::env::{self, current_exe};

// For loading .dic and .aff files
use std::collections::HashMap;
use std::time;

// For parsing .aff files
use regex::{Regex, bytes};

// Efficient implementation from Rust docs
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
// Capitalizes the first character in s - necessary since not all rules can be found in the lowercase variations
pub fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
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
        let start = time::Instant::now();
        let mut dictionary: HashMap<String, String> = HashMap::new();

        if let Ok(lines) = read_lines(dictpath) {

            for line in lines.flatten() {
                let record: Vec<&str> = line.split("/").collect();
                if record.len() == 2 {
                dictionary.insert(record[0].to_string(), record[1].to_string());
                }
            }
        }
        let elapsed_time = start.elapsed();
        println!(".dic file loaded in {} seconds", elapsed_time.as_secs_f32());
        dictionary
    }
    pub fn get_rules(&self, word: &str) -> Vec<char> {
        self.hashmap
            .get(word)
            .map(|s| s.chars().collect())
            .unwrap_or_else(|| vec![])

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
        let start = time::Instant::now();
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
                //println!("{:?}, group: {:?}", record, current_flag_char);
                }
            //println!("{:?}", rule_groups);
            }
        let elapsed_time = start.elapsed();
        println!(".aff file loaded in {} seconds", elapsed_time.as_secs_f32());
        rule_groups
        }

        pub fn apply_rule(&self, word: &str, rule_flag: char) -> Vec<String> {
            let mut declinations: Vec<String> = vec![];
            let result = self.groups.get(&rule_flag).unwrap();

            for rule in &result.rules {
                let formatted_remove_rule = format!("{}$", rule.condition);
                let re = Regex::new(&formatted_remove_rule).unwrap();

                if re.is_match(word) {
                    let remove_snippet = if &rule.remove == "0" { "" } else { &rule.remove };
                    let add_snippet = if &rule.add == "0" { "" } else { &rule.add };

                    let word_version = format!(
                                        "{}{}",
                                        word.strip_suffix(remove_snippet).unwrap_or_default(),
                                        add_snippet
                                    );
                    declinations.push(word_version);
                    //println!("{word_version} [for flag '{rule_flag}'] - Warunek: {formatted_remove_rule}");
                }
                //  else {
                //     let remove_snippet = if &rule.remove == "0" { "" } else { &rule.remove };
                //     let add_snippet = if &rule.add == "0" { "" } else { &rule.add };

                //     let word_version = format!(
                //                         "{}{}",
                //                         word.strip_suffix(remove_snippet).unwrap_or_default(),
                //                         add_snippet
                //                     );
                //     //println!("{word_version} [for flag '{rule_flag}'] <- NIEPOPRAWNE - Warunek: {formatted_remove_rule}");
                // }

            }

            declinations
            }


}

fn generate_full_dictionary(dict: &Dictionary, aff: &AffFile) -> HashMap<String, String> {

    let f = File::create("data/dictionary1.txt").unwrap();
    let mut writer = BufWriter::new(f);

    let full_dictionary: HashMap<String, String> = HashMap::new();

    for (word, rules) in &dict.hashmap {
        let flags: Vec<char> = rules.chars().collect();
        for flag in flags {
            let word_declinations = aff.apply_rule(word, flag);
            for declination in word_declinations {
                let record = format!("{},{}\n", declination, word);
                let result = writer.write(record.as_bytes());
                println!("{}: {} | Written {:?} bytes", declination, word, result);
            }

            //println!("{:?}", {aff.apply_rule(word, flag)})

        }
        //println!("{}: \"{:?}\"", word, rules_vec);
    }
    

    full_dictionary
}

fn read_full_dictionary(path: &str) -> HashMap<String, String> {
    let start = time::Instant::now();

    let mut full_dictionary: HashMap<String, String> = HashMap::new();

    if let Ok(lines) = read_lines(path) {

        for line in lines.flatten() {
            let record: Vec<&str> = line.split(",").collect();
            if record.len() == 2 {
            full_dictionary.insert(record[0].to_string(), record[1].to_string());
            }
        }
    }
    let elapsed_time = start.elapsed();
    println!("Full dictionary mapping file loaded in {} seconds", elapsed_time.as_secs_f32());
    full_dictionary
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

    // Full dictionary already generated
    //generate_full_dictionary(&dictionary, &aff_file);
    let full_map = read_full_dictionary("data/dictionary1.txt");

    loop {
        term.clear();
        println!("-----------------------------------");
        println!("Wpisz słowo, które chcesz wyszukać:");
        io::stdin().read_line(&mut term).expect("Błąd w odczytywaniu słowa");

        let start = time::Instant::now();
        let term = term.trim();
        let mut flags = dictionary.get_rules(&term);
        let mut capitalized_word_flags = dictionary.get_rules(&capitalize(&term));

        flags.append(&mut capitalized_word_flags);
        println!("Słowo: {}, Zasady: {:?}", term, flags);

        if flags.len() > 0 {
            for flag in flags {
                println!("{:?}", {aff_file.apply_rule(term, flag)})
            }
        } else {
            println!("{} to nie jest bazowe słowo, jego forma podstawowa to: {:?}", term, full_map.get(term));
        }


        let elapsed_time = start.elapsed();
        println!("Applying rules took {} seconds", elapsed_time.as_secs_f32())
    }

}
