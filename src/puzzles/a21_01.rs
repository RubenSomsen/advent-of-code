
pub fn run(file_location: &str) {
    println!("NOW RUNNING: {}", file!());

    let mut vec = parse_file(file_location);
    
    use std::time::Instant;
    let now = Instant::now();

    let a = answer_a(&mut vec);
    let b = answer_b(&mut vec);

    let elapsed = now.elapsed();
    println!("PERFORMANCE: {:?}", elapsed);

    println!("ANSWER A: {:?}", a);
    println!("ANSWER B: {:?}", b);
}

fn answer_a(vec: &mut Vec<u16>) -> u16 {
    let mut last = vec[0];
    let mut count = 0;
    for val in vec {
        if val>&mut last {
            count += 1;
        }
        last = *val;
    }
    return count
}

fn answer_b(vec: &mut Vec<u16>) -> u16 {
    let mut last = vec[0] + vec[1] + vec[2];
    let mut count = 0;
    for i in 1..vec.len()-2 {
        let val = vec[i] + vec[i+1] + vec[i+2];
        if val>last {
            count += 1;
        }
        last = val;
    }
    return count
}

fn parse_file(file_name: &str) -> Vec<u16> {
    use std::fs;
    let file = fs::read_to_string(file_name).expect("Unable to read file");

    let mut vec = vec![];
    for line in file.lines() {
        let val = line.parse().unwrap();
        vec.push(val);
    }
    return vec
}