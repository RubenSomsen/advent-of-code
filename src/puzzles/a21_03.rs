
pub fn run(file_location: &str) {
    println!("NOW RUNNING: {}", file!());

    let vec = parse_file(file_location);
    
    use std::time::Instant;
    let now = Instant::now();

    let a = answer_a(&vec);
    // call the function twice to get the answer for b
    let b = answer_b(&vec, 12, true); //*answer_b(&vec, 12, false);

    let elapsed = now.elapsed();
    println!("PERFORMANCE: {:?}", elapsed);

    println!("ANSWER A: {:?}", a);
    println!("ANSWER B: {:?}", b);
}

fn answer_a(vec: &Vec<u64>) -> u64 {
    let mut res: u64 = 0;
    for i in 0..12 { // 12 bits hard coded
        let mut count: u64 = 0;
        for val in vec {
            if *val & (1 << i) != 0 {
                count += 1;
            }
        }
        if count as usize > (1000/2) { // 1000 lines hard coded
            res += 1 << i;
        }
    }
    let res_opposite = res ^ (1 << 12)-1; // 12 bits hard coded
    return res*res_opposite
}

// vec = the initial vector, i = which bit to look at, b = whether the bit should be true/false
fn answer_b(vec: &Vec<u64>, i: u32, b: bool) -> u64 {

    // split up the result into two vectors
    let (mut true_vec, mut false_vec) = (vec![], vec![]);

    // walk through the initial vector
    for val in vec {
        // set vec to true_vec or false_vec depending on if the bit (i) is true or false
        let vec = if val & (1 << i-1) != 0 { &mut true_vec } else { &mut false_vec };
        // and push the value to that vector
        vec.push(*val);
    }
    // now set vec again, but pick true_vec/false_vec depending on which one is longer and bool b
    let vec = if (true_vec.len() >= false_vec.len()) == b { &true_vec } else { &false_vec };

    // performance speedup hack, skips one iteration (the longest one)
    if i == 12 {
        let vec2 = if (true_vec.len() >= false_vec.len()) == !b { &true_vec } else { &false_vec };
        return answer_b(vec, i-1, b)*answer_b(vec2, i-1, !b)
    }

    // run the function again for the next bit, unless vec.len() == 1, then return it as the answer
    return if vec.len() == 1 { vec[0] } else { answer_b(vec, i-1, b) }
}

fn parse_file(file_name: &str) -> Vec<u64> {
    use std::fs;
    let file = fs::read_to_string(file_name).expect("Unable to read file");

    let mut vec = vec![];
    for line in file.lines() { vec.push(u64::from_str_radix(line, 2).unwrap()); }
    return vec
}