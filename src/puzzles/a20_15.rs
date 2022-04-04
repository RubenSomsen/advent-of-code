use std::collections::HashMap;

pub fn run(file_location: &str) {
    println!("NOW RUNNING: {}", file!());

    let vector: Vec<u64> = parse_file(file_location);

    assert_eq!(reverse_search(&vec![(1,0),(2,1),(3,0),(4,1)]), 2);

    use std::time::Instant;
    let now = Instant::now();

    let a: u64 = slow_answer(&vector, 2020);
    let b: u64 = fastest_answer(&vector, 30000000); // takes ~4 sec in debug

    let elapsed = now.elapsed();
    println!("PERFORMANCE: {:?}", elapsed);

    println!("ANSWER A: {}", a);
    println!("ANSWER B: {}", b);
}

fn slow_answer(vector: &Vec<u64>, turns: u64) -> u64 {
    let mut vector2: Vec<(u64, u64)> = vec![];
    for i in 0..turns as usize {
        if i<vector.len() {
            vector2.push((i as u64+1, vector[i]));
        }
        else {
            vector2.push((i as u64+1, reverse_search(&vector2)));
        }
    }
    return vector2[vector2.len()-1].1
}

fn faster_answer(vector: &Vec<u64>, turns: u64) -> u64 {
    let mut hashmap: HashMap<u64,u64> = HashMap::new();
    let mut diff = 0;
    for i in 0..turns as usize-1 {
        if i<vector.len() { hashmap.insert(vector[i], i as u64+1); }
        else {
            let previous_value = hashmap.insert(diff, i as u64+1);
            diff = 0;
            if previous_value != None { diff = (i as u64+1) - previous_value.unwrap() }
        }
    }
    return diff
}

fn fastest_answer(vector: &Vec<u64>, turns: u64) -> u64 {
    let mut prev_vector: Vec<u64> = vec![0; turns as usize];
    let mut diff = 0;
    for i in 0..turns as usize-1 {
        if i<vector.len() { prev_vector[vector[i] as usize] = i as u64+1; }
        else {
            let prev_value = prev_vector[diff as usize];
            prev_vector[diff as usize] = i as u64+1;
            diff = 0;
            if prev_value != 0 { diff = (i as u64+1) - prev_value }
        }
    }
    return diff
}

fn reverse_search(vector: &Vec<(u64, u64)>) -> u64 {
    let index = vector[vector.len()-1].0;
    let number = vector[vector.len()-1].1;
    for i in 0..vector.len()-1 {
        let tuple = vector[vector.len()-2-i];
        if tuple.1 == number {
            return index-tuple.0
        }    
    }
    return 0
}

fn parse_file(file_name: &str) -> Vec<u64> {
    use std::fs;
    let file = fs::read_to_string(file_name).expect("Unable to read file");

    let mut vector: Vec<u64> = vec![]; 
    for number in file.split(",") { 
        let number: u64 = number.parse().unwrap();
        vector.push(number);
    }
    return vector
}