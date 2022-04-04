pub fn run(file_location: &str) {
    println!("NOW RUNNING: {}", file!());

    let values: Values = parse_file(file_location);
    
    use std::time::Instant;
    let now = Instant::now();
    
    // Answer A or B depends entirely on the data set
    let ab = answer(&values);

    let elapsed = now.elapsed();
    println!("PERFORMANCE: {:?}", elapsed);

    println!("ANSWER AB: {}", ab);
}

fn check_output(start_vector: &Vec<u8>, target_output: &Vec<u8>, values: &Values) -> (Vec<Vec<u8>>, bool) { 
    let mut or_vectors: Vec<Vec<u8>> = vec![];
    for i in 0..start_vector.len() {

        // checks for out of bounds, i.e. an invald output
        if i>=target_output.len() {
            return (or_vectors, false)
        }

        let value = start_vector[i];
        let target = target_output[i];

        // if value is neither target, insert it into the or_vector
        if !(value == values.target_a || value == values.target_b) {
            let comp = &values.comps[value as usize];
            or_vectors.push(insert_vector_at(&comp.vec_a, i, &start_vector));
            if comp.is_or {
                or_vectors.push(insert_vector_at(&comp.vec_b, i, &start_vector));
            }
            break // will loop again later, so everything will be checked
        }
        // if value isn't the exact target, return false (continue loop if it is)
        else if !(value == target) {
            return (or_vectors, false)
        }
    }
    if or_vectors.len()<1 { // bool needed to end loop and possibly return true
        return (or_vectors, start_vector.len() == target_output.len())
    }
    // replace each vec with the vecs you get back, IF bool is true (else keep)
    let mut next_vectors = vec![];
    for i in 0..or_vectors.len() {
        let (new_vec, has_ended) = check_output(&or_vectors[i], target_output, values);
        next_vectors.extend(new_vec.iter().cloned());
        if has_ended { 
            return (vec![or_vectors[i].clone()], true) 
        }
    }
    return (next_vectors, false)
}

// Works for both a and b, just the data set changes
fn answer(values: &Values) -> u16 {
    let mut answer = 0;
    for target_output in &values.outputs {
        let target_output = get_output_at_index(target_output, values);
        let start_vector = &values.comps[0].vec_a;

        let (set, is_valid) = check_output(start_vector, &target_output, values);
        if is_valid {
            answer += 1;
        }
    }
    return answer
}

// Alters the true false array into a vector with the numbers for "a" and "b" (parsing)
fn get_output_at_index(output: &Vec<bool>, values: &Values) -> Vec<u8> {
    let mut output_vector = vec![];
    for is_a in output {
        if *is_a { output_vector.push(values.target_a) }
        else { output_vector.push(values.target_b) }
    }
    return output_vector
}

// Takes out a value and inserts a vector there
fn insert_vector_at(insert_vector: &Vec<u8>, index: usize, target_vector: &Vec<u8>) -> Vec<u8> {
    let mut new_vec = vec![];
    let vec_a = &target_vector[0..index];
    let vec_b = &insert_vector;
    let vec_c = &target_vector[index+1..];
    new_vec.extend(vec_a.iter());
    new_vec.extend(vec_b.iter());
    new_vec.extend(vec_c.iter());
    return new_vec
}

fn parse_file(file_name: &str) -> Values {
    use std::fs;
    let file = fs::read_to_string(file_name).expect("Unable to read file");

    let mut values = Values { comps: vec![], outputs: vec![], target_a: 0, target_b: 0 };
    let mut file_split = file.split("\r\n\r\n");
    let input = file_split.next().unwrap();
    let output = file_split.next().unwrap();
    for line in input.lines() {
        let mut line_split = line.split(": ");
        let index: u8 = line_split.next().unwrap().parse().unwrap();
        let content = line_split.next().unwrap();
        let content_split = content.split(" ");
        let mut comp = Comp::new();
        for value in content_split {
            if value == "|" {
                comp.is_or = true;
                continue
            }
            else if value == "\"a\"" || value == "\"b\"" {
                if value == "\"a\"" { values.target_a = index; }
                else { values.target_b = index; }
                continue
                // an empty Comp will be present, but that's fine
            }
            if comp.is_or { comp.vec_b.push(value.parse().unwrap()); }
            else { comp.vec_a.push(value.parse().unwrap()); }
        }
        push_to_vector(&mut values.comps, index, comp);
    }
    for line in output.lines() {
        let mut bool_vector = vec![];
        for c in line.chars() {
            bool_vector.push(c == 'a');
        }
        values.outputs.push(bool_vector);
    }
    return values
}

// hack to fill up empty indices prior to insertion
fn push_to_vector(vector: &mut Vec<Comp>, index: u8, comp: Comp) {
    while vector.len()<1+index as usize { vector.push(Comp::new()) }
    vector[index as usize] = comp;
}

#[derive(Debug)]
struct Values {
    comps: Vec<Comp>, outputs: Vec<Vec<bool>>, target_a: u8, target_b: u8
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Comp {
    vec_a: Vec<u8>, vec_b: Vec<u8>, is_or: bool
}

impl Comp {
    fn new() -> Comp {
        return Comp { vec_a: vec![], vec_b: vec![], is_or: false }
    }
}