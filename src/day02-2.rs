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

        let position_a = line.get(0..position_dash).unwrap().parse::<usize>().unwrap() - 1;
        let position_b = line.get(position_dash+1..position_space).unwrap().parse::<usize>().unwrap() - 1;
        let character = line.get(position_colon-1..position_colon).unwrap();
        let password = line.get(position_colon+2..).unwrap();

        let is_it_at_a = character == password.get(position_a..position_a + 1).unwrap();
        let is_it_at_b = character == password.get(position_b..position_b + 1).unwrap();
        if is_it_at_a && !is_it_at_b ||
            !is_it_at_a && is_it_at_b {
            valid_password_count += 1;
        }
    }

    println!("Number of valid passwords: {}", valid_password_count);
}
