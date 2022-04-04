pub fn run(file_location: &str) {
    println!("NOW RUNNING: {}", file!());

    let vector: Vec<(String, i16, bool)> = parse_file(file_location);

    use std::time::Instant;
    let now = Instant::now();

    let (answer_a, is_infinite): (i16, bool) = answer_a(vector.clone());
    let answer_b: i16 = answer_b(vector);

    let elapsed = now.elapsed();
    println!("PERFORMANCE: {:?}", elapsed);

    println!("ANSWER A: {} {}", answer_a, is_infinite);
    println!("ANSWER B: {}", answer_b);
}

fn answer_a(mut code_vector: Vec<(String, i16, bool)>) -> (i16, bool) {
    let mut answer: i16 = 0;
    let mut i: usize = 0;
    while !code_vector[i].2 {
        code_vector[i].2 = true;
        let current = &code_vector[i];
        i += 1;
        if current.0 == "jmp" { i = (i as i16 + current.1 as i16 - 1) as usize; }
        else if current.0 == "acc" { answer += current.1 as i16; }
        if i>=code_vector.len() { return (answer, false) }
    }
    return (answer, true)
}

fn answer_b(code_vector: Vec<(String, i16, bool)>) -> i16 {
    for (i, code) in code_vector.iter().enumerate() {
        if code.0 == "acc" { continue }
        let mut current_code_vector = code_vector.clone(); // could avoid cloning so much but meh
        if code.0 == "jmp" { current_code_vector[i].0 = "nop".to_owned() }
        else { current_code_vector[i].0 = "jmp".to_owned() }
        let (current_answer, is_infinite): (i16, bool) = answer_a(current_code_vector);
        if !is_infinite { return current_answer }
    }
    return 0
}

fn parse_file(file_name: &str) -> Vec<(String, i16, bool)> {
    use std::fs;
    let file = fs::read_to_string(file_name).expect("Unable to read file");

    let mut code_vector: Vec<(String, i16, bool)> = vec![];
    for code in file.lines() {
        let mut code_split = code.split(" ");
        let code_name: &str = code_split.next().unwrap();
        let code_value: i16 = code_split.next().unwrap().parse().unwrap();
        code_vector.push((code_name.to_owned(), code_value, false));
    }
    return code_vector
}