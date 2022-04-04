use std::collections::HashMap;

pub fn run(file_location: &str) {
    println!("NOW RUNNING: {}", file!());

    let vector: Vec<(bool, u64, u64)> = parse_file(file_location);

    use std::time::Instant;
    let now = Instant::now();

    let a: u64 = answer(&vector, true);
    let b: u64 = answer(&vector, false); // NOTE: test input for A takes ~5 hours (too many X)

    let elapsed = now.elapsed();
    println!("PERFORMANCE: {:?}", elapsed);

    println!("ANSWER A: {}", a);
    println!("ANSWER B: {}", b);
}

// returns all variants with bits off, e.g. 101 (5) returns 000, 001, 100, 101 (0, 1, 4, 5)
fn get_bit_vector(value: u64) -> Vec<u64> {
    let mut bits:   Vec<u8>  = vec![];
    let mut values: Vec<u64> = vec![];
    for i in 0..36 {
        if value & (1 << i) != 0 {
            bits.push(i);
        }
    }
    let solutions = u64::pow(2, bits.len() as u32);
    for s in 0..solutions {
        let mut val: u64 = 0;
        for b in 0..36 {
            if s & (1 << b) != 0 {
                val += u64::pow(2, bits[b] as u32);
            }
        }
        values.push(val);
    }
    return values
}

fn answer(vector: &Vec<(bool, u64, u64)>, is_answer_a: bool) -> u64 {
    let mut hashmap: HashMap<u64, u64> = HashMap::new();
    let mut mask: (u64, u64, u64) = (0, !0, 0);
    for t in vector {
        if !t.0 {
            mask.0 = t.1; // 1 for every 1 (rest 0)
            mask.1 = t.2; // 0 for every 0 (rest 1)
            mask.2 = (!(t.1 ^ !t.2) << 28) >> 28; // 1 for every X (rest 0)
        }
        else if is_answer_a {
            let mut value = t.2;
            value |= mask.0;
            value &= mask.1; 
            hashmap.insert(t.1, value);
        }
        else {
            let mut value = t.1;
            value |= mask.0;
            value &= !mask.2;
            let values = get_bit_vector(mask.2);
            for val in values {
                hashmap.insert(value | val, t.2);
            }
        }
    }
    let mut answer: u64 = 0;
    for (k, v) in hashmap {
        answer += v;
    }
    return answer
}

fn parse_file(file_name: &str) -> Vec<(bool, u64, u64)> {
    use std::fs;
    let file = fs::read_to_string(file_name).expect("Unable to read file");

    let mut vector: Vec<(bool, u64, u64)> = vec![];
    for line in file.lines() {
        let mut t: (bool, u64, u64) = (true, 0, 0);
        let mut line_split = line.split(" = ");
        let line_code = line_split.next().unwrap();
        let line_value = line_split.next().unwrap();
        if line_code != "mask" { 
            let index = &line_code[4..line_code.len()-1];
            t.1 = index.parse().unwrap();
            t.2 = line_value.parse().unwrap();
        }
        else {
            t.0 = false;
            for c in line_value.chars() {
                t.1 <<= 1;
                t.2 <<= 1;
                if      c == '1' { t.1 += 1; }
                else if c == '0' { t.2 += 1; }
            }
            t.2 = !t.2;
        }
        vector.push(t);
    }
    return vector
}