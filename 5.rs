fn parse_to_buckets(input: &Vec<String>) -> Vec<String> {
    let mut rtn = vec![
        String::new(),
        String::new(),
        String::new(),
        String::new(),
        String::new(),
        String::new(),
        String::new(),
        String::new(),
        String::new(),
    ];
    for string in input {
        for (i, c) in string.chars().enumerate() {
            if i%4 == 1 {
                if c == '1' {
                    return rtn;
                } else if !c.is_whitespace() {
                    rtn[(i-1)/4].push(c);
                }
            }
        }
    }
    rtn
}

fn string_to_vector(s: String) -> Vec<String> {
    // Split the string on line breaks
    let lines = s.split("\n");

    // Convert the iterator of lines to a vector of strings
    let mut result = Vec::new();
    for line in lines {
        result.push(line.to_string());
    }

    result
}

fn get_commands(input: &Vec<String>) -> Vec<Vec<i32>> {
    let mut rtn = Vec::new();
    for string in input {
        if string.starts_with("move") {
            let commands=vec![string.split_whitespace().nth(1).unwrap().parse::<i32>().unwrap(),string.split_whitespace().nth(3).unwrap().parse::<i32>().unwrap(),string.split_whitespace().nth(5).unwrap().parse::<i32>().unwrap()];
            rtn.push(commands);
        }
    }
    rtn

}
fn remove_whitespace(s: &str) -> String {
    s.chars().filter(|c| !c.is_whitespace()).collect()
}

fn execute_crane(crates: &mut Vec<String>, commands: &Vec<Vec<i32>>) {
    for line in commands {
        for i in 0..line[0] {
            let block1 = crates[(line[1]as usize)-1].clone();
            let (c, t) = block1.split_at(1);
            let c = &remove_whitespace(&c);
            let t = &remove_whitespace(&t);
            crates[(line[1] as usize)-1]=t.to_string();
            println!("{}",crates[(line[1] as usize)-1]);
            crates[(line[2] as usize)-1].insert(0,c.chars().nth(0).unwrap());
            println!("{}",crates[(line[2] as usize)-1]);
            
        }
    }
    
}


fn main() {
    let content = std::fs::read_to_string("5.txt")
        .expect("file not found!");
    // Parse the string into a vector of integers
    let result= string_to_vector(content);
    let mut crates = parse_to_buckets(&result);
    let commands = get_commands(&result);
    execute_crane(&mut crates,&commands);
    for line in crates {
        let line = line.trim_start();
        print!("{}",&line.chars().nth(0).unwrap());
        
        
    }
}