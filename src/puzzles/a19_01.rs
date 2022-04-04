pub fn run(file_location: &str) {
    println!("NOW RUNNING: {}", file!());
    
    use std::time::Instant;
    let now = Instant::now();

    let a: usize = answer_a(&mut vector.clone());
    let b: usize = answer_b(&mut vector);

    let elapsed = now.elapsed();
    println!("PERFORMANCE: {:?}", elapsed);

    println!("ANSWER A: {}", a);
    println!("ANSWER B: {}", b);
}

fn answer_b(vector: &mut Vec<usize>) -> usize {
    for noun in 0..100 {
        for verb in 0..100 {
            let mut vector2 = vector.clone();
            vector2[1] = noun;
            vector2[2] = verb;
            if answer_a(&mut vector2) == 19690720 {
                return 100*noun+verb;
            }
        }
    }
    return 0
}

fn answer_a(vector: &mut Vec<usize>) -> usize {
    let mut i: usize = 0;
    while i<vector.len() {
        let input = vector[i];
        if input == 1 || input == 2 {
            let value1 = vector[vector[i+1]];
            let value2 = vector[vector[i+2]];
            let dest = vector[i+3];
            i += 4;
            if input == 1 {
                vector[dest] = value1 + value2;
            }
            else {
                vector[dest] = value1 * value2;
            }
        }
        else if input == 99 {
            break
        }
    }
    return vector[0]
}