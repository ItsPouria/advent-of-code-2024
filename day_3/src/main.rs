use regex::Regex;
use std::fs::read_to_string;
fn main() {
    let file_path = "src/input.txt";
    let re = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();
    let input: String = read_to_string(file_path).unwrap();
    let mut results: Vec<&str> = Vec::new();

    for cap in re.captures_iter(&input) {
        //println!("{}", cap.get(0).unwrap().as_str());
        results.push(cap.get(0).unwrap().as_str());
    }

    results.iter().map(|m| { 
       let nums: Vec<&str> = m[4..m.len()-1].split(',').collect();
       let num0: i32 = nums[0].parse().unwrap();
       let num1: i32 = nums[1].parse().unwrap();

       println!("{}", num0 * num1)
    });
}
