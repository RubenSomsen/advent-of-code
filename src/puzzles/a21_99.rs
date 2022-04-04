pub fn run(file_location: &str) {
    println!("NOW RUNNING: {}", file!());

    let mut vec = parse_file(file_location);
    
    use std::time::Instant;
    let now = Instant::now();

    let a = answer_a(&mut vec![]);
    let b = 0; //answer_b(&mut vec);

    let elapsed = now.elapsed();
    println!("PERFORMANCE: {:?}", elapsed);

    println!("ANSWER A: {:?}", a);
    println!("ANSWER B: {:?}", b);
}

fn answer_a(vec: &mut Vec<usize>) -> usize {
    let mut counter = 0;
    
    return counter
}

fn parse_file(file_name: &str) -> Vec<usize> {
    use std::fs;
    let file = fs::read_to_string(file_name).unwrap();
    
    return vec![]
}

struct Thing {
    state: Vec<usize>
}

impl Thing {
    fn new() -> Thing {
        return Thing { state: vec![] }
    }
}