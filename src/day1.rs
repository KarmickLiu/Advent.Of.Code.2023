use std::{fs, collections::HashMap};
use fancy_regex::Regex;

fn main() {
    let file_path = "./src/input/day1.txt";
    
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut summed: u32 = 0;

    for raw_line in contents.lines() {
        println!("{} is the original string", raw_line);
        
        let re = Regex::new("(?=(one|two|three|four|five|six|seven|eight|nine|\\d))").unwrap();
        let mut captures = re.captures_iter(raw_line);

        let first = captures.nth(0).unwrap().unwrap().get(1).unwrap().as_str();
        summed = summed + add_digit(first) * 10;

        let last : &str;
        if let Some(next) = captures.last() {
            last = next.unwrap().get(1).unwrap().as_str();
        } else {
            last = &first;
        }
        summed = summed + add_digit(last);

        println!("({} * 10 + {}) needs to be added", first, last);
        println!("{}", summed);
    }
}

fn add_digit(s: &str) -> u32 {
    let digit_words: HashMap<&str, u32> = HashMap::from([
        ("one",     1),
        ("two",     2), 
        ("three",   3), 
        ("four",    4), 
        ("five",    5), 
        ("six",     6), 
        ("seven",   7), 
        ("eight",   8), 
        ("nine",    9)
    ]);

    if let Some(d) = digit_words.get(s) {
        *d
    } else {
        s.parse::<u32>().unwrap()
    }
} 
