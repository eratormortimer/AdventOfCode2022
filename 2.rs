use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
fn get_points(attack: &str, reaction: &str) -> i32{
    let mut rtn = 0;
    if attack == "A" {
        if reaction == "X" {
            rtn += 3;
        } else if reaction == "Y" {
            rtn += 4;
        } else {
            rtn += 8;
        }
    } else if attack == "B" {
        if reaction == "X" {
            rtn += 1;
        } else if reaction == "Y" {
            rtn += 5;
        } else {
            rtn += 9;
        }
    } else if attack == "C" {
        if reaction == "X" {
            rtn += 2;
        } else if reaction == "Y" {
            rtn += 6;
        } else {
            rtn += 7;
        }
    }
   
    rtn
}

fn main() {
    let file = File::open("input_2.txt")
    .expect("file not found!");
    let  buf_reader = BufReader::new(file);
    let mut points = 0;
    for line in buf_reader.lines() {
        let line = line.unwrap();
        let line: Vec<&str> = line.split(" ").collect();
        points += get_points(line[0],line[1]);
    }
    println!( "{}",points);
}