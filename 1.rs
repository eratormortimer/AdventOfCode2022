use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
fn main()-> std::io::Result<()> {
    let file = File::open("input_1.txt")
    .expect("file not found!");
    let  buf_reader = BufReader::new(file);
    let mut counter=0;
    let mut array: [i32; 3] = [0; 3];
    let mut pointer = 0;
    for line in buf_reader.lines() {
        let line = line.unwrap();
        if line.is_empty() {
            if counter > array[pointer] {
                if array[pointer] == 0 {
                    array[pointer] = counter;
                    pointer+=1;
                    if pointer ==3{
                       pointer = array.iter().position(|&x| x == *array.iter().min().unwrap()).unwrap();
                    }
                    
                } else {
                    array[pointer] = counter;
                    pointer = array.iter().position(|&x| x == *array.iter().min().unwrap()).unwrap();
                }
                
            }
            counter = 0;
        } else {
            counter+=line.parse::<i32>().unwrap();
        }
    }
    println!( "{},{}",  *array.iter().min().unwrap(),array[0]+array[1]+array[2]);
    Ok(())
}