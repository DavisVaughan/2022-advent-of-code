use std::fs;

fn main() {
    let input = fs::read_to_string("data/input.txt").expect("Can't read file.");

    // - Split by blank lines to get "chunks" of values
    // - Split each chunk by line and parse each individual value, then sum them
    // - Collect sums into a vector
    let mut totals: Vec<u32> = input
        .split("\n\n")
        .map(|chunk| -> u32 {
            chunk
                .split("\n")
                .map(|x| x.parse::<u32>().expect("Can't parse value."))
                .sum()
        })
        .collect();
    
    // Sort in descending order and sum top 3
    totals.sort();
    totals.reverse();
    let out: u32 = totals[0..3].iter().sum();

    println!("{out}");
}
