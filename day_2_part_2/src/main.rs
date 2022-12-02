// X - lose, Y - draw, Z - win

use std::fs;

fn choice(a: u8, obj: u8) -> u8 {
    match obj {
        0 => if a == 0 { 2 } else { a - 1 },
        1 => a,
        2 => if a == 2 { 0 } else { a + 1 },
        _ => panic!("Unexpected objective")
    }
}

fn main() {
    let data = fs::read_to_string("input.txt").expect("Unable to find input.txt file");

    let points: u64 = data.split('\n').map_while(|a| {
        let mut words = a.split(' ').flat_map(|w| w.as_bytes().get(0)).take(2);

        // A -> 0, B -> 1, C -> 2
        let a = words.next().map(|w| w - 'A' as u8)?;

        // X -> 0, Y -> 1, Z -> 2
        let b = words.next().map(|w| w - 'X' as u8)?;

        assert!(a <= 2, "First player move is invalid");

        let choice = choice(a, b);

        let score = (choice + 1 + b * 3) as u64;

        Some(score)
    }).sum();

    println!("{}", points);
}
