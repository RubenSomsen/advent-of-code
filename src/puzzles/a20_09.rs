pub fn run(file_location: &str) {
    println!("NOW RUNNING: {}", file!());

    let vector: Vec<u64> = parse_file(file_location);

    use std::time::Instant;
    let now = Instant::now();

    let answer_a: u64 = answer_a(&vector);
    let answer_b: u64 = answer_b(&vector, answer_a);

    let elapsed = now.elapsed();
    println!("PERFORMANCE: {:?}", elapsed);

    println!("ANSWER A: {}", answer_a);
    println!("ANSWER B: {}", answer_b);
}

fn answer_a(number_vector: &Vec<u64>) -> u64 {
    let limit: usize = 25;
    let mut last_vector: Vec<u64> = vec![0; limit];
    for (i, number) in number_vector.iter().enumerate() {
        if !(i<limit) {
            if !get_numbers(&last_vector).contains(number) {
                return *number
            }
        }
        last_vector[i%limit] = *number;
    }
    return 0
}

fn answer_b(number_vector: &Vec<u64>, target: u64) -> u64 {
    let (mut i1, mut i2, mut total): (usize, usize, u64) = (0, 1, number_vector[0]);
    while total != target {
        total += number_vector[i2];
        if total > target {
            total -= number_vector[i1];
            total -= number_vector[i2];
            i1 += 1;
            i2 -= 1;
        }
        i2 += 1;
    }
    let slice = &number_vector[i1..i2-1];
    let min: u64 = *slice.iter().min().unwrap();
    let max: u64 = *slice.iter().max().unwrap();
    return  min+max
}

fn get_numbers(number_vector: &Vec<u64>) -> Vec<u64> {
    let mut addition_vector: Vec<u64> = vec![];
    for (i1, number1) in number_vector.iter().enumerate() {
        for (i2, number2) in number_vector.iter().enumerate() {
            if i1>=i2 { continue }
            addition_vector.push(number1+number2);
        }
    }
    return addition_vector
}

fn parse_file(file_name: &str) -> Vec<u64> {
    use std::fs;
    let file = fs::read_to_string(file_name).expect("Unable to read file");

    let mut number_vector: Vec<u64> = vec![];
    for number in file.lines() {
        number_vector.push(number.parse().unwrap());
    }
    return number_vector
}