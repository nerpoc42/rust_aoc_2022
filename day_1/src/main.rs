use std::fs;

fn main() {
    let data = fs::read_to_string("input.txt").expect("Unable to find input.txt file");

    let max = data.split("\n\n").map(|line| {
        line.split_whitespace()
            .map(|w| w.parse::<u64>().expect("Not a positive integer in file"))
            .sum::<u64>()
    }).max();

    match max {
        Some(max) => println!("Max: {}", max),
        None => println!("No input found")
    };
}
