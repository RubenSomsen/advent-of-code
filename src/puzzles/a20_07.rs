use std::collections::HashMap;

pub fn run(file_location: &str) {
    println!("NOW RUNNING: {}", file!());

    let hashmap: HashMap<String, HashMap<String, u8>> = parse_file(file_location);
    let target: &str = "shiny gold";

    use std::time::Instant;
    let now = Instant::now();

    let answer_a: u16 = answer_a(&hashmap, target);
    let answer_b: u32 = answer_b(&hashmap, target);

    let elapsed = now.elapsed();
    println!("PERFORMANCE: {:?}", elapsed);

    println!("ANSWER A: {}", answer_a);
    println!("ANSWER B: {}", answer_b);
}

fn answer_a(bag_hashmap: &HashMap<String, HashMap<String, u8>>, target: &str) -> u16 {
    let mut answer: u16 = 0;
    for bag in bag_hashmap.keys() {
        answer += bag_contains_target(bag, bag_hashmap, target) as u16;
    }
    return answer
}

fn bag_contains_target(bag: &str, bag_hashmap: &HashMap<String, HashMap<String, u8>>, target: &str) -> bool {
    let contents_hashmap: &HashMap<String, u8> = bag_hashmap.get(bag).unwrap();
    if contents_hashmap.contains_key(target) { 
        return true
    }
    else {
        for bag2 in contents_hashmap.keys() {
            if bag_contains_target(bag2, bag_hashmap, target) {
                return true
            }
        }
    }
    return false
}

fn answer_b(bag_hashmap: &HashMap<String, HashMap<String, u8>>, target: &str) -> u32 {
    let mut answer: u32 = 0;
    let contents_hashmap: &HashMap<String, u8> = bag_hashmap.get(target).unwrap();
    for k in contents_hashmap.keys() {
        let v: u8 = *contents_hashmap.get(k).unwrap();
        answer += v as u32 + v as u32*answer_b(bag_hashmap, k);
    }
    return answer
}

fn parse_file(file_name: &str) -> HashMap<String, HashMap<String, u8>> {
    use std::fs;
    let file = fs::read_to_string(file_name).expect("Unable to read file");

    let mut bag_hashmap: HashMap<String, HashMap<String, u8>> = HashMap::new();
    for bag in file.lines() {
        let mut bag_split = bag.split(" bags contain ");
        let mut contents_hashmap: HashMap<String, u8> = HashMap::new();
        let bag_name: &str = bag_split.next().unwrap();
        let bag_contents: &str = bag_split.next().unwrap();
        if bag_contents != "no other bags." { 
            for bag2 in bag_contents.split(", ") {
                let mut contents = bag2.split_ascii_whitespace();
                let amount: u8 = contents.next().unwrap().parse().unwrap();
                let w1: &str = contents.next().unwrap();
                let w2: &str = contents.next().unwrap();
                let w0: String = w1.to_owned() + " " + w2;
                contents_hashmap.insert(w0, amount);
            }
        }
        bag_hashmap.insert(bag_name.to_owned(), contents_hashmap);        
    }
    return bag_hashmap
}