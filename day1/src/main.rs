use std::fs;

fn main() {
    let mut val = 0;
    let mut result = vec![];

    let input = fs::read_to_string("./input.txt").unwrap();
    for line in input.lines() {
        if line == "" {
            result.push(val);
            val = 0;
            continue;
        }

        val += line.parse::<i32>().unwrap();
    }

    result.sort();

    println!("{:?}", result.iter().rev().take(3).sum::<i32>());
}
