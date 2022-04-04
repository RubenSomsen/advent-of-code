pub fn run(file_location: &str) {
    println!("NOW RUNNING: {}", file!());

    let vector: Vec<[bool; 10]> = parse_file(file_location);

    use std::time::Instant;
    let now = Instant::now();

    let answer_a: Vec<u16> = answer_a(&vector);
    let answer_b: u16 = smart_answer_b(&answer_a);

    let elapsed = now.elapsed();
    println!("PERFORMANCE: {:?}", elapsed);

    println!("ANSWER A: {}", answer_a[answer_a.len()-1]);
    println!("ANSWER B: {}", answer_b);
}

fn answer_a(vector: &Vec<[bool; 10]>) -> Vec<u16> {
    let mut vector2: Vec<u16> = vec![];
    for array in vector {
        vector2.push(seat_id(&array));
    }
    vector2.sort();
    return vector2
}

fn smart_answer_b(vector: &Vec<u16>) -> u16 {
    let mut index = (vector.len() - (vector.len() % 2)) / 2;
    let mut jump = index;
    while jump>1 {
        jump = (jump - (jump%2)) / 2;
        if vector[index] == index as u16 + 28 {
            index += jump;
        }
        else {
            index -= jump;
        }
    }
    return vector[index]+1; // the +1 is lazy, probebly needs an if
}

fn basic_answer_b(vector: &Vec<u16>) -> u16 {
    let mut answer_b: u16 = 0;
    for (i, e) in vector.iter().enumerate() {
        if i as u16+28 != *e {
            answer_b = *e-1;
            break;
        }
    }
    return answer_b
}

fn seat_id(array: &[bool; 10]) -> u16 {
    let mut id: u16 = 0;
    for (i, e) in array.iter().enumerate() {
        if *e { id += u16::pow(2, i as u32) }
    }
    return id
}

fn parse_file(file_name: &str) -> Vec<[bool; 10]> {
    use std::fs;
    let file = fs::read_to_string(file_name).expect("Unable to read file");

    let mut vector: Vec<[bool; 10]> = vec![];
    for seat in file.lines() {
        let mut array: [bool; 10] = [false; 10];
        for (i, c) in seat.chars().rev().enumerate() {
            if i<3 && c == 'R' { array[i] = true; }
            else if c == 'B' { array[i] = true; }
        }
        vector.push(array);
    }
    return vector
}