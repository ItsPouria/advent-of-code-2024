use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("src/locations.txt")?;
    let reader = BufReader::new(file);

    let mut left_numbers: Vec<u32> = Vec::new();
    let mut right_numbers: Vec<u32> = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let numbers: Vec<&str> = line.split_whitespace().collect();

        if numbers.len() >= 2 {
            let left_num = numbers[0].parse::<u32>()?;
            let right_num = numbers[1].parse::<u32>()?;
            left_numbers.push(left_num);
            right_numbers.push(right_num);
        } else {
            eprintln!("Skipping line: '{}' - Not enough numbers", line);
        }
    }

    left_numbers.sort();
    right_numbers.sort();
    let mut result = 0;
    for i in 0..left_numbers.len() {
        if left_numbers[i] > right_numbers[i] {
            result += left_numbers[i] - right_numbers[i];
        } else {
            result += right_numbers[i] - left_numbers[i];
        }
    }
    println!("{}", result);

    Ok(())
}
