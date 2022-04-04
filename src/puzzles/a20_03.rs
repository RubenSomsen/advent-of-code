pub fn run(file_location: &str) {
    println!("NOW RUNNING: {}", file!());

    let vector: Vec<Vec<bool>> = parse_file(file_location);
    let array: [[u8; 2]; 5] = [[1, 1], [3, 1], [5, 1], [7, 1], [1, 2]];

    use std::time::Instant;
    let now = Instant::now();

    let answer_a: u16 = answer_a(&vector, 3, 1);
    let answer_b: u32 = answer_b(&vector, array);

    let elapsed = now.elapsed();
    println!("PERFORMANCE: {:?}", elapsed);

    println!("ANSWER A: {}", answer_a);
    println!("ANSWER B: {}", answer_b);
}

fn answer_a(vector: &Vec<Vec<bool>>, move_right: u8, move_down: u8) -> u16 {
    let mut answer: u16 = 0;
    let mut state: u8 = 0;
    for (index, inner_vector) in vector.iter().enumerate() {
        if index == 0 || index%(move_down as usize) != 0 { continue; }
        state = (state+move_right)%(inner_vector.len() as u8);  
        let is_tree = inner_vector[state as usize];
        if is_tree { answer += 1; }
    }
    return answer
}

fn answer_b(vector: &Vec<Vec<bool>>, array: [[u8; 2]; 5]) -> u32 {
    let mut answer: u32 = 0;
    for inner_array in array.iter() {
        let value = answer_a(&vector, inner_array[0], inner_array[1]);
        if answer == 0 { answer = value as u32; }
        else { answer *= value as u32; }
    }
    return answer
}

fn parse_file(file_name: &str) -> Vec<Vec<bool>> {
    use std::fs;
    let file = fs::read_to_string(file_name).expect("Unable to read file");

    let mut outer_vector: Vec<Vec<bool>> = vec![];
    for sentence in file.lines() {
        let mut inner_vector: Vec<bool> = vec![];
        for character in sentence.chars() {
            let mut is_tree: bool = false;
            if character == '#' { is_tree = true; }
            inner_vector.push(is_tree);
        }
        outer_vector.push(inner_vector);
    }
    return outer_vector
}
