use std::fs;
use std::io;
use std::io::BufRead;

fn main() {
    let path = String::from("data/input.txt");
    let file = fs::File::open(path).expect("Can't open file.");
    let reader = io::BufReader::new(file);

    let mut out: i64 = 0;
    let mut current: i64 = 0;
    
    for line in reader.lines() {
        let line = line.expect("Can't read line.");
        
        if line.is_empty() {
            if current > out {
                out = current;
            }
            current = 0;
            continue;
        }

        let value: i64 = line.parse().expect("Can't parse line.");
        current += value;
    }

    println!("{out}");
}
