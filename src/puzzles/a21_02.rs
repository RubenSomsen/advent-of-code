
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

fn answer_a(vec: &mut Vec<(String, u64)>) -> u64 {

    let mut depth = 0;
    let mut horiz = 0;

    for tuple in vec {
        if tuple.0 == "up" {
            depth -= tuple.1;
        }
        else if tuple.0 == "down" {
            depth += tuple.1;
        }
        else {
            horiz += tuple.1;
        }
    }

    return depth*horiz;
}

fn answer_b(vec: &mut Vec<(String, u64)>) -> u64 {

    let mut aim = 0;
    let mut depth = 0;
    let mut horiz = 0;

    for tuple in vec {
        if tuple.0 == "up" {
            aim -= tuple.1;
        }
        else if tuple.0 == "down" {
            aim += tuple.1;
        }
        else {
            horiz += tuple.1;
            depth += tuple.1*aim;
        }
    }

    return depth*horiz;
}

fn parse_file(file_name: &str) -> Vec<(String, u64)> {
    use std::fs;
    let file = fs::read_to_string(file_name).expect("Unable to read file");

    let mut vec = vec![];
    for line in file.lines() {
        let mut parts = line.split(" ");
        let part1 = &mut parts.next().unwrap();
        let part2 = *&mut parts.next().unwrap().parse().unwrap();
        vec.push((part1.to_owned(), part2));
    }
    println!("{:?}", vec);
    return vec
}