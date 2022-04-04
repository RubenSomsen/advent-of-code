pub fn run(file_location: &str) {
    println!("NOW RUNNING: {}", file!());

    let mut tuple: (Vec<Limitation>, Vec<Vec<u16>>) = parse_file(file_location);

    use std::time::Instant;
    let now = Instant::now();

    let a: u16 = answer_a(&tuple);
    discard_invalid_tickets(&mut tuple);
    let b: u64 = answer_b(&tuple);

    let elapsed = now.elapsed();
    println!("PERFORMANCE: {:?}", elapsed);

    println!("ANSWER A: {}", a);
    println!("ANSWER B: {}", b);
}

fn answer_b(tuple: &(Vec<Limitation>, Vec<Vec<u16>>)) -> u64 {
    let mut answer: u64 = 1;
    let mut vector = get_possible_names(tuple);
    cull_double_names(&mut vector);
    for (i, field) in vector.iter().enumerate() {
        let number = tuple.1[0][i];
        if &field[0][0..3] == "dep" {
            answer *= number as u64;
        }
    }
    return answer
}

// takes out all the double names, leaving us with one name per field
fn cull_double_names(field_vector: &mut Vec<Vec<String>>) {
    let mut checked: Vec<bool> = vec![false; field_vector.len()];
    loop {
        for i1 in 0..field_vector.len() {
            if !checked[i1] && field_vector[i1].len() == 1 {
                checked[i1] = true;
                // remove element from all but checked
                for i2 in 0..field_vector.len() {
                    if !checked[i2] {
                        for i3 in 0..field_vector[i2].len() {
                            if field_vector[i2][i3] == field_vector[i1][0] {
                                field_vector[i2].swap_remove(i3);
                                break
                            }
                        }
                        //field_vector[i2].retain(|x| *x != field_vector[i2][0]); // could be faster, but meh
                    }
                }
            }
        }
        // if checked == true everywhere, break loop
        let mut c = true;
        for i in 0..checked.len() { c = c && checked[i]; }
        if c { break }
    }
}


// returns the possible name for every field on the tickets (multiple answers)
fn get_possible_names(tuple: &(Vec<Limitation>, Vec<Vec<u16>>)) -> Vec<Vec<String>> {
    let mut field_vector: Vec<Vec<String>> = vec![];
    let (limitations, ticket_vector) = (&tuple.0, &tuple.1);
    for i in 0..ticket_vector[0].len() {   
        let mut name_vector: Vec<String> = vec![];
        'a: for range in limitations { 
            for ticket in ticket_vector {
                let number = ticket[i];
                if !is_in_range(number, range) {
                    continue 'a
                }
            }
            name_vector.push(range.name.to_owned());
        }
        field_vector.push(name_vector);
    }
    return field_vector
}

fn answer_a(tuple: &(Vec<Limitation>, Vec<Vec<u16>>)) -> u16 {
    let mut answer: u16 = 0;
    let (limitations, ticket_vector) = (&tuple.0, &tuple.1);
    for ticket in ticket_vector {
        for number in ticket {
            let mut in_range = false;
            for range in limitations {
                if is_in_range(*number, range) {
                    in_range = true;
                    break
                }
            }
            if !in_range { answer += number }
        }
    }
    return answer
}

fn discard_invalid_tickets(tuple: &mut (Vec<Limitation>, Vec<Vec<u16>>)) {
    let mut new_ticket_vector: Vec<Vec<u16>> = vec![]; 
    let (limitations, ticket_vector) = (&tuple.0, &tuple.1);
    'a: for ticket in ticket_vector {
        for number in ticket {
            let mut in_range = false;
            for range in limitations {
                if is_in_range(*number, range) {
                    in_range = true;
                    break
                }
            }
            if !in_range { continue 'a }
        }
        new_ticket_vector.push(ticket.clone());
    }
    tuple.1 = new_ticket_vector;
}

fn is_in_range(number: u16, range: &Limitation) -> bool {
    if (number>=range.from1 && number<=range.to1) || (number>=range.from2 && number<=range.to2) {
        return true
    }
    return false
}

fn parse_file(file_name: &str) -> (Vec<Limitation>, Vec<Vec<u16>>) {
    use std::fs;
    let file = fs::read_to_string(file_name).expect("Unable to read file");

    let mut tuple: (Vec<Limitation>, Vec<Vec<u16>>) = (vec![], vec![]);

    let mut part = file.split("\r\n\r\n");
    let limitations = part.next().unwrap();
    let yours = part.next().unwrap().lines().nth(1).unwrap();
    let nearby = part.next().unwrap();
    for line in limitations.lines() {
        let mut limi = Limitation { name: String::new(), from1: 0, to1: 0, from2: 0, to2: 0 };
        let mut parts = line.split(": ");
        limi.name = parts.next().unwrap().to_owned();
        let mut limits = parts.next().unwrap().split(" or ");
        let mut l1 = limits.next().unwrap().split("-");
        limi.from1 = l1.next().unwrap().parse().unwrap();
        limi.to1 = l1.next().unwrap().parse().unwrap();
        let mut l2 = limits.next().unwrap().split("-");
        limi.from2 = l2.next().unwrap().parse().unwrap();
        limi.to2 = l2.next().unwrap().parse().unwrap();
        tuple.0.push(limi);
    }
    for (i, mut line) in nearby.lines().enumerate() {
        if i==0 { line = yours }
        let mut vec: Vec<u16> = vec![];
        for value in line.split(",") {
            vec.push(value.parse().unwrap());
        }
        tuple.1.push(vec);
    }
    return tuple
}

#[derive(Debug)]
struct Limitation {
    name: String, from1: u16, to1: u16, from2: u16, to2: u16
}