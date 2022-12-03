
use std::collections::HashSet;

fn share_char(a: &str, b: &str) -> char {
    let set1: HashSet<char> = a.chars().collect();
    let set2: HashSet<char> = b.chars().collect();
    *(&set1 & &set2).iter().next().unwrap()
}

fn main() {
    let content = std::fs::read_to_string("input_3.txt")
        .expect("file not found!");
    let mut points = 0;
    for line in content.lines() {
        let (split1, split2) = line.split_at(line.len()/2);
        points += share_char(split1, split2) as u32 - 64;
    }
    println!("{}", points);
}