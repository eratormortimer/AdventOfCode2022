
use std::collections::HashSet;
use std::ops::BitAnd;

fn share_char(a: &str, b: &str, c: &str) -> char {
    let set1: HashSet<char> = a.chars().collect();
    let set2: HashSet<char> = b.chars().collect();
    let set3: HashSet<char> = c.chars().collect();
    *set1.bitand(&set2).bitand(&set3).iter().next().unwrap()
    //set1.intersection(&set2).collect::<HashSet<char>>().iter().next().unwrap()


}

fn main() {
    let content = std::fs::read_to_string("input_3.txt")
        .expect("file not found!");
    let mut points = 0;
    let lines : Vec<_> = content.lines().collect();
    
    for line in lines.chunks(3) {
        let char_result= share_char(line[0], line[1], line[2]) as u32;
        match char_result {
            97..=122 => points += char_result - (70 + 26),
            _ => points += char_result - 64 + 26,
        }
    }

    println!("{}", points);
}