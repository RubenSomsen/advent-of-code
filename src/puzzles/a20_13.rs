pub fn run(file_location: &str) {
    println!("NOW RUNNING: {}", file!());

    let mut vector: (u64, Vec<u64>) = parse_file(file_location);

    // assert_eq!(answer_b(&mut(0, vec![7,5,0,0,9]), 0, 1), 14);
    // assert_eq!(faster_answer_b(&mut(0, vec![7,5,0,0,9])), 14);

    use std::time::Instant;
    let now = Instant::now();

    let a: u64 = answer_a(&mut vector);
    let b: u64 = fast_answer_b(&mut vector);

    let elapsed = now.elapsed();
    println!("PERFORMANCE: {:?}", elapsed);

    println!("ANSWER A: {}", a);
    println!("ANSWER B: {}", b);
}

fn fast_answer_b(tuple: &mut (u64, Vec<u64>)) -> u64 {
    let mut b_vector = vec![];
    let vector = &tuple.1.clone();
    for (i, value) in vector.iter().enumerate() {
        if *value != 0 {
            b_vector.push((*value,i));
        }
    }
    b_vector.sort();
    b_vector.reverse();
    // highest first, slightly faster

    let mut incr = b_vector[0].0;
    let mut time = incr - b_vector[0].1 as u64;
    for i in 1..b_vector.len() {
        let bus = b_vector[i];
        while (time+bus.1 as u64)%bus.0 != 0 { time += incr; }
        incr *= b_vector[i].0;
    }
    return time
}

fn answer_a(tuple: &mut (u64, Vec<u64>)) -> u64 {
    let (time, busses) = (tuple.0, &tuple.1);
    let mut answers: Vec<(u64, u64)> = vec![];
    for bus in busses {
        if *bus != 0 {
            answers.push((bus - time%bus, *bus));
       }
    }
    let mut ans = (9999, 0); // 9999 here is lazy, but it works
    for answer in answers {
        if answer.0 < ans.0 {
            ans = answer;
        }
    }
    return ans.0*ans.1
}

fn parse_file(file_name: &str) -> (u64, Vec<u64>) {
    use std::fs;
    let file = fs::read_to_string(file_name).expect("Unable to read file");

    let mut vector: Vec<u64> = vec![];
    let mut lines = file.lines();
    let time: u64 = lines.next().unwrap().parse().unwrap();
    let busses = lines.next().unwrap().split(",");
    for bus in busses {
        if bus != "x" {
            vector.push(bus.parse().unwrap());
        }
        else {
            vector.push(0);
        }
    }
    return (time, vector)
}