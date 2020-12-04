use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "day1.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut numbers = Vec::new();

    for (_, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        numbers.push(line.parse::<i32>().unwrap())
    }

    for i in 0..numbers.len() {
        for j in i..numbers.len() {
            if numbers[i] + numbers[j] == 2020 {
                println!("{}", numbers[i] * numbers[j]);
            }
        }
    }
}
