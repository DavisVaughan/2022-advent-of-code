use std::fs;

fn normalize(x: u8) -> u8 {
    if x >= 97 {
        x - 97
    } else {
        x - 65 + 26
    }
}

fn find_duplicated(x: &[u8]) -> u8 {
    // Small enough that we can allocate this at
    // each iteration with basically no cost
    let mut exists: [bool; 52] = [false; 52];

    let size = x.len();
    let split = size / 2;

    // Use cheap reference views into `x`
    let first = &x[0..split];
    let second = &x[split..size];

    // Mark seen items in the rucksack
    for x in first {
        let index = normalize(*x);
        exists[index as usize] = true;
    }

    // Find the previously seen item
    for x in second {
        let index = normalize(*x);
        if exists[index as usize] {
            return index + 1;
        }
    }

    // The puzzle asserts there is exactly 1 duplicate item
    panic!("Never reached.");
}

fn main() {
    // `fs::read()` goes straight into a `Vec<u8>`
    let input = fs::read("data/input.txt").expect("File must be readable.");

    let out: u32 = input
        .split(|b| *b == b'\n')
        .map(|line| find_duplicated(line) as u32)
        .sum();

    println!("{out}");
}
