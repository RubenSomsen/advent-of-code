pub fn run(file_location: &str) {
    println!("NOW RUNNING: {}", file!());

    let vector: Vec<(u8, u8, char, String)> = parse_file(file_location);
    
    use std::time::Instant;
    let now = Instant::now();

    let answer_a: u16 = answer_a(&vector);
    let answer_b: u16 = answer_b(&vector);

    let elapsed = now.elapsed();
    println!("PERFORMANCE: {:?}", elapsed);

    println!("ANSWER A: {}", answer_a);
    println!("ANSWER B: {}", answer_b);
}

fn answer_a(vector: &Vec<(u8, u8, char, String)>) -> u16 {
    let mut answer: u16 = 0;
    for tuple in vector.iter() {
        let count = tuple.3.matches(tuple.2).count() as u8;
        if count >= tuple.0 && count <= tuple.1 {
            answer += 1;
        }
    }
    return answer
}

fn answer_b(vector: &Vec<(u8, u8, char, String)>) -> u16 {
    let mut answer: u16 = 0;
    for tuple in vector.iter() {
        let check1: bool = tuple.2 == tuple.3.chars().nth(tuple.0 as usize -1).unwrap();
        let check2: bool = tuple.2 == tuple.3.chars().nth(tuple.1 as usize -1).unwrap();
        if check1 != check2 {
            answer += 1;
        }
    }
    return answer
}

fn parse_file(file_name: &str) -> Vec<(u8, u8, char, String)> {
    use std::fs;
    let file = fs::read_to_string(file_name).expect("Unable to read file");

    let mut vector: Vec<(u8, u8, char, String)> = vec![];
    for sentence in file.lines() {
        let mut parts = sentence.split_ascii_whitespace();
        let mut numbers = parts.next().unwrap().split('-');
        let number1: u8 = numbers.next().unwrap().parse::<u8>().unwrap();
        let number2: u8 = numbers.next().unwrap().parse::<u8>().unwrap();
        let target: char = parts.next().unwrap().chars().next().unwrap();
        let phrase: String = parts.next().unwrap().to_string();
        let tuple: (u8, u8, char, String) = (number1, number2, target, phrase);
        vector.push(tuple);
    }
    return vector
}
