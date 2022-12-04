use std::ops::RangeInclusive;

trait RangeExt {
    fn contains_range(&self, other: RangeInclusive<i32>) -> bool;
    fn overlaps_range(&self, other: RangeInclusive<i32>) -> bool;
}
impl RangeExt for RangeInclusive<i32> {
    fn contains_range(&self, other: RangeInclusive<i32>) -> bool {
        self.contains(other.start()) && self.contains(other.end()) 
        || other.contains(self.start()) && other.contains(self.end())
    }
    
    fn overlaps_range(&self, other: RangeInclusive<i32>) -> bool { 
        self.contains(other.start()) || self.contains(other.end()) 
        || other.contains(self.start()) || other.contains(self.end())
    }
}

fn main() {
    let content = std::fs::read_to_string("input_4.txt")
        .expect("file not found!");
    let mut points = 0;
    for line in content.lines() {
        let mut ranges = line.split(',')
            .map(|x| {
                let mut split = x.split('-').map(|y| y.parse::<i32>().unwrap());
                let a = split.next().unwrap();
                let b = split.next().unwrap();
                a..=b
            });
        let first = ranges.next().unwrap();
        let second = ranges.next().unwrap();
        points += first.contains_range(second) as i32;
    }
    println!("{}", points);
}