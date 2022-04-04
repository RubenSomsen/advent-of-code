use std::collections::HashMap;

pub fn run(file_location: &str) {
    println!("NOW RUNNING: {}", file!());
 
    let (pair_map, input) = parse_file(file_location);
    let mut memo = HashMap::new();
    
    use std::time::Instant;
    let now = Instant::now();

    let a = answer(&pair_map, &input, 10, &mut memo);
    let b = answer(&pair_map, &input, 40, &mut memo);

    let elapsed = now.elapsed();
    println!("PERFORMANCE: {:?}", elapsed);

    println!("ANSWER A: {:?}", a);
    println!("ANSWER B: {:?}", b);
}

fn answer(pair_map: &HashMap<u8,u8>,input: &Vec<u8>, iterations: u8, memo: &mut HashMap<u64,[u64;10]>) -> u64 {
    let mut tally = [0; 10];
    for i in 0..input.len()-1 {
        let a = input[i];
        let b = input[i+1];
        tally[a as usize] += 1;
        let tally_i = count(pair_map, a, b, iterations, memo);
        tally.iter_mut().zip(tally_i.iter()).for_each(|(v1, v2)| *v1 += v2);
    }
    tally[input[input.len()-1] as usize] += 1;
    tally.sort();
    return tally[tally.len()-1] - tally[0]
}

fn count(pair_map: &HashMap<u8,u8>, a: u8, c: u8, iterations: u8, memo: &mut HashMap<u64,[u64;10]>) -> [u64; 10] {
    let pair_key = a << 4 | c;
    let memo_key = (iterations as u64) << 8 | pair_key as u64;
    let check = memo.get(&memo_key);
    if check != None { return *check.unwrap() }
    let b = *pair_map.get(&pair_key).unwrap();
    let mut tally = [0; 10];
    tally[b as usize] += 1;
    if iterations != 1 {
        let tally_l = count(pair_map, a, b, iterations-1, memo);
        let tally_r = count(pair_map, b, c, iterations-1, memo);
        tally.iter_mut().zip(tally_l.iter()).for_each(|(v1, v2)| *v1 += v2);
        tally.iter_mut().zip(tally_r.iter()).for_each(|(v1, v2)| *v1 += v2);
    }
    memo.insert(memo_key, tally);
    return tally
} 

fn parse_file(file_name: &str) -> (HashMap<u8,u8>, Vec<u8>) {
    use std::fs;
    let file = fs::read_to_string(file_name).expect("Unable to read file");
    let unique: Vec<char> = "BCFHKNOPSV".chars().collect(); // lazily hard coded unique chars
    let mut pair_map = HashMap::new();
    let mut split = file.split("\r\n\r\n");
    let chars: Vec<char> = split.next().unwrap().chars().collect();
    let mut input = vec![];
    for c in chars { input.push(unique.iter().position(|&x| x == c).unwrap() as u8) }
    for line in split.next().unwrap().lines() {
        let mut split = line.split(" -> ");
        let (mut first, mut second) = (split.next().unwrap().chars(), split.next().unwrap().chars());
        let (a, b, c) = (first.next().unwrap(), first.next().unwrap(), second.next().unwrap());
        let key_a = unique.iter().position(|&x| x == a).unwrap() as u8;
        let key_b = unique.iter().position(|&x| x == b).unwrap() as u8;
        let value = unique.iter().position(|&x| x == c).unwrap() as u8;
        pair_map.insert(key_a << 4 | key_b, value);
    }
    return (pair_map, input)
}