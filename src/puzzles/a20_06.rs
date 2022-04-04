pub fn run(file_location: &str) {
    println!("NOW RUNNING: {}", file!());

    let vector: Vec<Vec<[bool; 26]>> = parse_file(file_location);

    use std::time::Instant;
    let now = Instant::now();

    let answer_a: u16 = answer(&vector, false);
    let answer_b: u16 = answer(&vector, true);

    let elapsed = now.elapsed();
    println!("PERFORMANCE: {:?}", elapsed);

    println!("ANSWER A: {}", answer_a);
    println!("ANSWER B: {}", answer_b);
}

fn answer(group_vector: &Vec<Vec<[bool; 26]>>, is_answer_b: bool) -> u16 {
    let mut answer: u16 = 0;
    for group in group_vector {
        for i in 0..26 { 
            let mut boolean: bool = is_answer_b;
            for person in group {
                if person[i] == !is_answer_b {
                    boolean = person[i];
                }
            }
            answer += boolean as u16;
        }
    }
    return answer
}

fn parse_file(file_name: &str) -> Vec<Vec<[bool; 26]>> {
    use std::fs;
    let file = fs::read_to_string(file_name).expect("Unable to read file");

    let mut group_vector: Vec<Vec<[bool; 26]>> = vec![];
    let groups = file.split("\r\n\r\n");
    for group in groups {
        let mut person_vector: Vec<[bool; 26]> = vec![];
        let people = group.split("\r\n");
        for person in people {
            let mut answer_array: [bool; 26] = [false; 26];
            for c in person.chars() {
                
                let i: u8 = c as u8 - 'a' as u8;
                answer_array[i as usize] = true;
            }
            person_vector.push(answer_array)
        }
        group_vector.push(person_vector);
    }
    return group_vector
}