use std::{fs, collections::HashMap};
use fancy_regex::Regex;

fn main() {
    let file_path = "./src/input/day2.txt";
    
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut games:Vec<u32> = Vec::new(); 

    let mut powers:Vec<u32> = Vec::new();

    for raw_line in contents.lines() {
        let game_num = raw_line.split(": ").next().unwrap().split(" ").nth(1).unwrap().parse::<u32>().unwrap();

        let sets = raw_line.split(": ").nth(1).unwrap();

        let mut max_cube:HashMap<&str, u32> = HashMap::from([
            ("red",   0),
            ("green", 0), 
            ("blue",  0)
        ]);

        for set in sets.split("; ") {
            let re = Regex::new("(((?:1[3-9]|[2-9]\\d) red)|((?:1[4-9]|[2-9]\\d) green)|((?:1[5-9]|[2-9]\\d) blue))").unwrap();
            if let Some(capture) = re.captures_iter(set).next() {
                if !(games.contains(&game_num)) {
                    games.push(game_num);
                }
            }

            for cubes in set.split(", ") {
                let cube_num = cubes.split(" ").next().unwrap().parse::<u32>().unwrap();
                for colour in ["red", "green", "blue"] {
                    if cubes.contains(colour) && (cube_num > *max_cube.get(colour).unwrap()) {
                        max_cube.insert(colour, cube_num);
                    }
                }
            }
        }
        
        let mut power:u32 = 1; 
        for (k, v) in max_cube {
            power = power * v;
        }
        powers.push(power);
    }

    // Part 1: I misread the question and got the cheating games instead. Fixed it by some quick math hack :P
    let cheated: u32 = games.iter().sum();
    println!("{:?}", 101*100/2 - cheated);

    // Part 2
    let power_sum: u32 = powers.iter().sum();
    println!("{}", power_sum);
}
