use std::fs;

fn normalize(x: u8) -> u8 {
    if x >= 97 {
        x - 97
    } else {
        x - 65 + 26
    }
}

fn main() {
    // `fs::read()` goes straight into a `Vec<u8>`
    let input = fs::read("data/input.txt").expect("File must be readable.");
    let input = input.split(|b| *b == b'\n');

    let mut out: u32 = 0;

    // On a `count` of 3 we find the duplicated element
    let mut count: u8 = 0;

    const SIZE: usize = 52;
    let mut times: [u8; SIZE] = [0; SIZE];
    let mut exists: [bool; SIZE] = [false; SIZE];

    for line in input {
        count += 1;

        // Mark seen elements
        for x in line {
            let index = normalize(*x);
            exists[index as usize] = true;
        }

        // If seen at all, increment by 1
        for x in 0..SIZE {
            if exists[x] {
                times[x] += 1;
            }
        }

        exists.fill(false);

        if count == 3 {
            // Find element duplicated across all 3
            for (i, &x) in times.iter().enumerate() {
                if x == 3 {
                    out += (i + 1) as u32;
                    break;
                }
            }

            count = 0;
            times.fill(0);
        }
    }

    println!("{out}");
}
