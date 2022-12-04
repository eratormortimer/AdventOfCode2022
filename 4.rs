
fn calc_containment(section1: &str, section2: &str) -> i32 {
    let bounds1: Vec<i32> = section1.split("-").map(|x| x.parse::<i32>().unwrap()).collect();
    let bounds2: Vec<i32> = section2.split("-").map(|x| x.parse::<i32>().unwrap()).collect();
    if bounds1[0] <= bounds2[0] && bounds1[1] >= bounds2[1] || bounds2[0] <= bounds1[0] && bounds2[1] >= bounds1[1] {
        return 1;
    }
    0
    
}
fn calc_overlap(section1: &str, section2: &str) -> i32 {
    let bounds1: Vec<i32> = section1.split("-").map(|x| x.parse::<i32>().unwrap()).collect();
    let bounds2: Vec<i32> = section2.split("-").map(|x| x.parse::<i32>().unwrap()).collect();
    if bounds1[0] <= bounds2[0] && bounds1[1] >= bounds2[1] || bounds2[0] <= bounds1[0] && bounds2[1] >= bounds1[1] 
        || bounds1[0] <= bounds2[0] && bounds2[0] <= bounds1[1] || bounds1[0]<= bounds2[1] && bounds2[1] <= bounds1[1] 
        || bounds2[0] <= bounds1[0] && bounds1[0] <= bounds2[1] || bounds2[0]<= bounds1[1] && bounds1[1] <= bounds2[1] 
    {
        return 1;
    }
    0
}




fn main() {
    let content = std::fs::read_to_string("input_4.txt")
        .expect("file not found!");
    let mut points = 0;
    for line in content.lines() {
        let line: Vec<&str> = line.split(",").collect();
        points += calc_overlap(line[0], line[1]);
    }
    println!("{}", points);
}