pub fn run(file_location: &str) {
    println!("NOW RUNNING: {}", file!());

    let mut vector: Vec<u64> = parse_file(file_location);
    vector.sort();
    vector.push(vector[vector.len()-1] + 3); // add final plug

    use std::time::Instant;
    let now = Instant::now();

    let answer_a: u32 = answer_a(&vector);
    let answer_b: u64 = answer_b(&vector);

    let elapsed = now.elapsed();
    println!("PERFORMANCE: {:?}", elapsed);

    println!("ANSWER A: {}", answer_a);
    println!("ANSWER B: {}", answer_b);
}

fn answer_a(number_vector: &Vec<u64>) -> u32 {
    let mut d1: u32 = 0;
    let mut d3: u32 = 1;
    for (i, number) in number_vector.iter().enumerate() {
        if i==0 { 
            if number_vector[i] == 1 { d1 += 1 }
            if number_vector[i] == 3 { d3 += 1 }
        }
        else if number-number_vector[i-1] == 1 { d1 += 1 }
        else if number-number_vector[i-1] == 3 { d3 += 1 }
    }
    return d1*d3
}

fn answer_b(number_vector: &Vec<u64>) -> u64 {
    let mut vector: Vec<u16> = vec![];
    let mut counter: u16 = 0;
    for (i, number) in number_vector.iter().enumerate() {
        if i==0 && number_vector[i] == 1 { counter += 1; }
        else if number-number_vector[i-1] == 1 { counter += 1; }
        else if number-number_vector[i-1] == 3 {
            if counter > 1 {
                vector.push(counter-1);
            }
            counter = 0;
         }
    }
    vector.sort();
    let possibility_vector: Vec<u16> = get_possibility_vector(vector[vector.len() as usize-1]);
    let mut answer: u64 = 1;
    for number in vector {
        answer *= possibility_vector[number as usize -1] as u64;
    }
    return answer
}

fn get_possibility_vector(length: u16) -> Vec<u16> {
    let mut vector: Vec<u16> = vec![2,4,7,13];
    for i in 4..length { // untested... turns out length never goes past 3
        vector.push(vector[i as usize-1]*2-vector[i as usize-4]);
    }
    return vector
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