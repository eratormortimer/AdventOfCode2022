use std::collections::HashSet;
fn chars_until_repeat(input: &str, message_length: usize) -> i32 {
    let input: Vec<char> = input.chars().collect::<Vec<_>>();
    let mut cache: Vec<char> = vec![];
    let mut set = HashSet::new();
    let mut rtn = 0;
    for character in input {
        if cache.len() == message_length {
            let removed_char = cache.drain(0..1).collect::<Vec<char>>()[0];
            if !cache.contains(&removed_char) {
                set.remove(&removed_char);
            }
            
            println!("{removed_char}");
            
        }
        cache.push(character);
        set.insert(character);
        rtn += 1;
        if set.len() == message_length {
            return rtn;
        }
    }
    rtn
}

fn main() {
    let content = std::fs::read_to_string("input_6.txt")
        .expect("file not found!");
    // Parse the string into a vector of integers
    let result = chars_until_repeat(&content, 14); 

    println!("{result}");
}