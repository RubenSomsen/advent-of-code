pub fn run(file_location: &str) {
    println!("NOW RUNNING: {}", file!());

    let mut vec = parse_file(file_location);
    let sum: usize = vec.iter().sum();
    println!("mean: {:?}", sum/vec.len()); // warning: rounds down
    
    use std::time::Instant;
    let now = Instant::now();

    let target_a = vec[vec.len()/2];
    let min_range = vec[0];
    let max_range = vec[vec.len()-1];
    let a = answer_a(&mut vec, target_a, false);
    let b = answer_b(&mut vec, min_range, max_range);

    let elapsed = now.elapsed();
    println!("PERFORMANCE: {:?}", elapsed);

    println!("ANSWER A: {:?}", a);
    println!("ANSWER B: {:?}", b);
}

fn answer_a(vec: &mut Vec<usize>, target: usize, exp: bool) -> usize {
    let mut counter = 0;
    for v in vec {
        let mut val = if *v<target { target-*v } else { *v-target };
        if exp { val = (val*(val+1))/2 }; //val = (val - (val&1) + 1) * (((val - (val&1)) / 2) + (val&1)); }
        counter += val;
    }
    return counter
}

fn answer_b(vec: &mut Vec<usize>, min_range: usize, max_range: usize) -> usize {
    let mut target = max_range - min_range;
    if target == 0 {
        println!("ans {}", min_range); // apparently this is just the mean
        return answer_a(vec, min_range, true) }
    target = (target - (target&1)) / 2 + min_range;
    let one = answer_a(vec, target, true);
    let two = answer_a(vec, target+1, true);
    if one<two { return answer_b(vec, min_range, target); }
    else { return answer_b(vec, target+1, max_range); }
}

fn parse_file(file_name: &str) -> Vec<usize> {
    use std::fs;
    let file = fs::read_to_string(file_name).expect("Unable to read file");

    let mut vec: Vec<usize> = file.split(",").map(|v| v.parse().unwrap()).collect();
    vec.sort();
    return vec
}
