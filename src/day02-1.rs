use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "input/day2.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut valid_password_count = 0;
    for (_, line) in reader.lines().enumerate() {
        let line = line.unwrap();

        let position_colon = line.find(':').unwrap();
        let position_dash = line.find('-').unwrap();
        let position_space = line.find(' ').unwrap();

        let minimum = line.get(0..position_dash).unwrap().parse::<i32>().unwrap();
        let maximum = line.get(position_dash+1..position_space).unwrap().parse::<i32>().unwrap();
        let character = line.get(position_colon-1..position_colon).unwrap().chars().next().unwrap();
        let password = line.get(position_colon+2..).unwrap();

        let mut count = 0;

        for c in password.chars() {
            if c == character {
                count += 1;
            }
        }

        if count >= minimum && count <= maximum {
            valid_password_count += 1;
        }
    }

    println!("Number of valid passwords: {}", valid_password_count);
}
