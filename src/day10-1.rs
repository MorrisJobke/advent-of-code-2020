use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "input/day10.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut numbers = Vec::new();

    for (_, line) in reader.lines().enumerate() {
        let number = line.unwrap().parse::<i32>().unwrap();
        numbers.push(number);
    }

    numbers.sort();
    numbers.push(numbers[numbers.len() - 1] + 3);

    let mut number_of_1_steps = 0;
    let mut number_of_3_steps = 0;
    let mut previous_number = 0;
    for number in numbers {
        let diff = number - previous_number;
        println!("Prev: {}, Number: {}, Diff: {}", previous_number, number, diff);
        previous_number = number;
        if diff == 1 {
            number_of_1_steps += 1;
        }
        if diff == 3 {
            number_of_3_steps += 1;
        }
    }

    println!("1 jolt: {}, 3 jolts: {}", number_of_1_steps, number_of_3_steps);
    println!("Result: {}", number_of_1_steps * number_of_3_steps);
}
