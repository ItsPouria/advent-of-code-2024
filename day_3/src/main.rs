use regex::Regex;
use std::fs::read_to_string;

fn main() -> std::io::Result<()> {
    let file_path = "src/input.txt";
    let re_num = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let input: String = read_to_string(file_path).unwrap();
    let mut results: i32 = 0;

    for cap in re_num.captures_iter(&input) {
        let num1 = cap.get(1).unwrap().as_str().parse::<i32>().unwrap();
        let num2 = cap.get(2).unwrap().as_str().parse::<i32>().unwrap();
        results += num1 * num2;
    }
    println!("results for part one: {}", results);

    let re2 = Regex::new(
        r"(?<do>do\(\))|(?<dont>don't\(\))|mul\((?<op1>[0-9]{1,3}),(?<op2>[0-9]{1,3})\)",
    )
    .unwrap();

    let mut enabled = true;
    let sum: u32 = re2
        .captures_iter(&input)
        .filter_map(|cap| {
            let r#match = &cap[0];

            match (r#match, enabled) {
                ("do()", _) => {
                    enabled = true;
                    None
                }
                ("don't()", _) => {
                    enabled = false;
                    None
                }
                (_, true) => {
                    let op1 = cap["op1"].parse::<u32>().ok()?;
                    let op2 = cap["op2"].parse::<u32>().ok()?;
                    Some(op1 * op2)
                }
                _ => None,
            }
        })
        .sum();
    println!("answer for part 2 is: {}", sum);
    println!("{}", enabled);
    Ok(())
}
