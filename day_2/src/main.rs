use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut safe_case = 0;
    if let Ok(lines) = read_lines("src/input.txt") {
        for line in lines.map_while(Result::ok) {
            let numbers: Vec<&str> = line.split_whitespace().collect();
            let numbers: Vec<u32> = numbers
                .into_iter()
                .map(|x| x.parse::<u32>().unwrap())
                .collect();
            if check_order(&numbers) == "safe" {
                safe_case += 1;
            } else if check_order(&numbers) == "unsafe" {
                for i in 0..numbers.len() {
                    let mut cloned = numbers.clone();
                    _ = cloned.remove(i);
                    if check_order(&cloned) == "safe" {
                        safe_case += 1;
                        break;
                    }
                }
            }
        }
        println!("total safe {} ", safe_case);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn check_order(slice: &[u32]) -> &'static str {
    // Check if the slice is sorted in ascending order
    let is_ascending = slice.windows(2).all(|w| w[0] <= w[1]);
    // Check if the slice is sorted in descending order
    let is_descending = slice.windows(2).all(|w| w[0] >= w[1]);
    let is_within_range = slice.windows(2).all(|w| {
        let diff = (w[0] as i32 - w[1] as i32).abs();
        diff >= 1 && diff <= 3
    });
    // If all elements are in ascending order, return "Ascending order!"
    if is_ascending && is_within_range || is_descending && is_within_range {
        "safe"
    // If all elements are in descending order, return "Descending order!"
    } else {
        "unsafe"
    }
}
