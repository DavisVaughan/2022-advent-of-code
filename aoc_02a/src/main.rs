use std::fs;
use std::collections::HashMap;

struct Game {
    win: u32,
    me: u32
}

fn main() {
    let input = fs::read_to_string("data/input.txt").expect("Can't read file.");

    let mut map = HashMap::new();

    map.insert(String::from("A X"), Game { win: 3, me: 1});
    map.insert(String::from("A Y"), Game { win: 6, me: 2});
    map.insert(String::from("A Z"), Game { win: 0, me: 3});
    map.insert(String::from("B X"), Game { win: 0, me: 1});
    map.insert(String::from("B Y"), Game { win: 3, me: 2});
    map.insert(String::from("B Z"), Game { win: 6, me: 3});
    map.insert(String::from("C X"), Game { win: 6, me: 1});
    map.insert(String::from("C Y"), Game { win: 0, me: 2});
    map.insert(String::from("C Z"), Game { win: 3, me: 3});
    
    let out: u32 = input
        .lines()
        .map(|x| -> u32 {
            let game = map.get(x).expect("Unknown combination encountered.");
            game.me + game.win
        })
        .sum();

    println!("{out}");
}
