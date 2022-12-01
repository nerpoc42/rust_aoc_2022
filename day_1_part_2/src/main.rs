use std::fs;

fn main() {
    let data = fs::read_to_string("input.txt").expect("Unable to find input.txt file");

    let mut calories: Vec<u64> = data.split("\n\n").map(|line| {
        line.split_whitespace()
            .map(|w| w.parse::<u64>().expect("Not a positive integer in file"))
            .sum::<u64>()
    }).collect();

    calories.sort_by(|a, b| b.cmp(a));

    let total: u64 = calories.iter().take(3).sum();

    println!("Total calories for 3 max elves: {}", total);
}
