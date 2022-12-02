use std::fs;

fn score(a: u8, b: u8) -> u8 {
    if a == b {
        3
    } else if a == b + 1 || b == a + 2 {
        6
    } else {
        0
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
        assert!(b <= 2, "Second player move is invalid");

        let score = (b + 1 + score(b, a)) as u64;

        Some(score)
    }).sum();

    println!("{}", points);
}
