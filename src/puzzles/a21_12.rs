use std::collections::HashMap;

pub fn run(file_location: &str) {
    println!("NOW RUNNING: {}", file!());

    let (cave_vec, start_vec) = parse_file(file_location);
    let mut memo = HashMap::new();
    
    use std::time::Instant;
    let now = Instant::now();

    let a = answer(&cave_vec, &start_vec, true, &mut memo);
    let b = answer(&cave_vec, &start_vec, false, &mut memo);

    let elapsed = now.elapsed();
    println!("PERFORMANCE: {:?}", elapsed);

    println!("ANSWER A: {:?}", a);
    println!("ANSWER B: {:?}", b);
}

// run depth_first_search for each index in starting_vec
fn answer(cave_vec: &Vec<Cave>, start_vec: &Vec<usize>, answer_a: bool, memo: &mut HashMap<u16,usize>) -> usize {
    let mut counter = 0;
    for index in start_vec {
        counter += depth_first_search(cave_vec, *index, 0, answer_a, memo);
    }
    return counter
}

// go through Caves from a certain index, while keeping track of visited small caves
fn depth_first_search(cave_vec: &Vec<Cave>, index: usize, mut state: u8, revisit: bool, memo: &mut HashMap<u16,usize>) -> usize {
    
    // update the state if you're entering a small cave for the first time
    if cave_vec[index].is_small && state & (1 << index) == 0 { state += 1 << index }
    
    // reuse any previously calculated answer (memoization, ~10x speedup)
    let memo_val = ((revisit as u16) << 15 | (index as u16) << 8) | (state as u16);
    let check = memo.get(&memo_val);
    if check != None { return *check.unwrap() }

    // get a neighbor
    let mut counter = 0;
    for i in 0..cave_vec[index].neighbors.len() {
        let i_neighbor = cave_vec[index].neighbors[i];

        // if neighbor is "end" (always last entry in vector)
        if i_neighbor == cave_vec.len()-1 { counter += 1 } 
        
        // if large cave or 1st time small cave
        else if !cave_vec[i_neighbor].is_small || state & (1 << i_neighbor) == 0 { 
            counter += depth_first_search(cave_vec, i_neighbor, state, revisit, memo);
        }

        // if 2nd time small cave (only for one small cave, and only for answer b)
        else if !revisit { 
        counter += depth_first_search(cave_vec, i_neighbor, state, true, memo);
        }   
    }
    memo.insert(memo_val, counter);
    return counter
}

fn parse_file(file_name: &str) -> (Vec<Cave>, Vec<usize>) {
    use std::fs;
    let file = fs::read_to_string(file_name).expect("Unable to read file");

    // create name_vec of unique sorted names
    let mut name_vec = vec![];
    for line in file.lines() {
        let mut split = line.split("-");
        for _ in 0..2 {
            let ab = split.next().unwrap();
            if ab != "end" && ab != "start" { name_vec.push(ab) }
        }
    } // sort by small caves first, deduplicate, and put "end" cave in last
    name_vec.sort(); name_vec.reverse(); name_vec.dedup(); name_vec.push("end");

    // fill cave_vec with Caves
    let mut cave_vec = vec![];
    for i in 0..name_vec.len() {
        cave_vec.push(Cave::new((name_vec[i].chars().next().unwrap() as u8) > 95));
    }

    // fill the Caves with neighbors and fill the start_vec with starting indexes
    let mut start_vec = vec![];
    for line in file.lines() {
        let mut split = line.split("-");
        let (a, b) = (split.next().unwrap(), split.next().unwrap());
        if a != "start" && b != "start" { 
            let ia = name_vec.iter().position(|&r| r == a).unwrap();
            let ib = name_vec.iter().position(|&r| r == b).unwrap();
            cave_vec[ia].neighbors.push(ib);
            cave_vec[ib].neighbors.push(ia);
        }
        else {
            let ab = if a == "start" { b } else { a };
            let iab = name_vec.iter().position(|&r| r == ab).unwrap();
            start_vec.push(iab);
            
        }
    }
    return (cave_vec, start_vec)
}

struct Cave {
    is_small: bool, neighbors: Vec<usize>
}

impl Cave {
    fn new(is_small: bool) -> Cave {
        return Cave { is_small, neighbors: vec![] }
    }
}