
use std::collections::HashSet;

fn share_char(a: &str, b: &str, c: &str) -> char {
    let set1: HashSet<char> = a.chars().collect();
    let set2: HashSet<char> = b.chars().collect();
    let set3: HashSet<char> = c.chars().collect();
    *( &(&set1 & &set2) & &set3).iter().next().unwrap()
}

fn main() {
    let content = std::fs::read_to_string("input_3.txt")
        .expect("file not found!");
    let mut points = 0;
    let mut count = 0;
    let mut strings = Vec::new();
    for line in content.lines() {
        strings.push(line);
        count += 1;
        if count == 3 {
            let char_result= share_char(strings[0], strings[1], strings[2]) as u32;
            match char_result {
                97..=122 => points += char_result - (70 + 26),
                _ => points += char_result - 64 + 26,
            }
            strings.clear();
            count = 0;
        }
       
    }
    println!("{}", points);
}