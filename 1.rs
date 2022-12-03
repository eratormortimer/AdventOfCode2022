use std::io::prelude::*;

fn main() {
    let content = std::fs::read_to_string("input_1.txt")
        .expect("file not found!");
    
    let mut counter = 0;
    let mut array: [i32; 3] = [0; 3];
    let mut index = 0;
    
    for line in content.lines() {
        if line.is_empty() {
            if counter > array[index] {
                array[index] = counter;
                let min_element = *array.iter().min().unwrap();
                index = array.iter().position(|&x| x == min_element).unwrap();
            }
            counter = 0;
        } else {
            counter += line.parse::<i32>().unwrap();
        }
    }
    println!("{},{},{}", array[0],array[1],array[2]);
}