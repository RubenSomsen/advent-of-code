pub fn run(file_location: &str) {
    println!("NOW RUNNING: {}", file!());

    let mut vec = parse_file(file_location);
    
    use std::time::Instant;
    let now = Instant::now();

    let (a, b) = answer(&mut vec);

    let elapsed = now.elapsed();
    println!("PERFORMANCE: {:?}", elapsed);

    println!("ANSWER A: {:?}", a);
    println!("ANSWER B: {:?}", b);
}

fn answer(vec: &mut Vec<usize>) -> (usize, usize) {
    let mut a = 0;
    for i in 0..256 {
        vec[(7+i)%9] += vec[i%9];
        if i == 80-1 { a = vec.iter().sum(); }
    }
    return (a, vec.iter().sum())
}

fn parse_file(file_name: &str) -> Vec<usize> {
    use std::fs;
    let file = fs::read_to_string(file_name).expect("Unable to read file");

    let vec: Vec<usize> = file.split(",").map(|v| v.parse().unwrap()).collect();
    let mut fish_vec = vec![0; 9];

    for v in vec {
        fish_vec[v] += 1;
    }
    return fish_vec
}
