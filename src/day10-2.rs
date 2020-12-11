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

    let mut previous_number = 0;
    let mut ones_in_a_row = 0;

    let mut permutations = Vec::new();
    for number in numbers {
        let diff = number - previous_number;
        println!("Prev: {}, Number: {}, Diff: {}", previous_number, number, diff);
        previous_number = number;
        if diff == 1 {
            ones_in_a_row += 1;
        }
        if diff == 3 {
            /*    options  not allowed  
            1 ->  2        0
            2 ->  4        0
            3 ->  2^3-1    1 
            4 ->  2^4-3    3
            5 ->  2^5-6    6
            */
            let permutations_per_segment = match ones_in_a_row {
                0 => 1,
                1 => 1,
                2 => 2,
                3 => 4,
                4 => 7,
                5 => 13,
                6 => 26,
                _ => 0,
            };

            println!("Permutations in this segment: {} (1s in a row: {})", permutations_per_segment, ones_in_a_row);
            permutations.push(permutations_per_segment);
            ones_in_a_row = 0;
        }
    }

    let mut result = 1 as i64;
    for p in permutations {
        result *= p;
    }

    println!("Result: {}", result);
}

