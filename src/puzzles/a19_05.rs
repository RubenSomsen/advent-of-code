pub fn run(file_location: &str) {
    println!("NOW RUNNING: {}", file!());

    let mut vector: Vec<i32> = parse_file(file_location);
    
    use std::time::Instant;
    let now = Instant::now();

    let a: Vec<i32> = answer(&mut vector.clone(), 1);
    let b: Vec<i32> = answer(&mut vector, 5);

    let elapsed = now.elapsed();
    println!("PERFORMANCE: {:?}", elapsed);

    println!("ANSWER A: {:?}", a);
    println!("ANSWER B: {:?}", b);
}

fn answer(vector: &mut Vec<i32>, input: i32) -> Vec<i32> {
    let mut i: i32 = 0;
    let mut output_vector: Vec<i32> = vec![];
    while (i as usize)<vector.len() {
        let mut op_code = vector[i as usize];

        //println!("! {:?}", op_code);
        let mut modes: [bool; 3] = [false; 3];

        if op_code > 10000 { 
            op_code -= 10000;
            modes[2] = true;
         }
        if op_code > 1000 { 
            op_code -= 1000;
            modes[1] = true; 
        }
        if op_code > 100 { 
            op_code -= 100;
            modes[0] = true;
        }

        if op_code == 1 || op_code == 2 {
            let value1 = if modes[0] { vector[i as usize+1] } else { vector[vector[i as usize+1] as usize] };
            let value2 = if modes[1] { vector[i as usize+2] } else { vector[vector[i as usize+2] as usize] };
            let dest = vector[i as usize+3];
            i += 4;
            if op_code == 1 {
                vector[dest as usize] = value1 + value2;
            }
            else {
                vector[dest as usize] = value1 * value2;
            }
        }
        else if op_code == 3 {
            let dest = vector[i as usize+1];
            vector[dest as usize] = input;
            i += 2;
        }
        else if op_code == 4 { 
            let dest = vector[i as usize+1];
            let value = if modes[0] { dest } else { vector[dest as usize] };
            output_vector.push(value);
            //println!("Output: {:?}", value);
            i += 2;
        }
        else if op_code == 5 || op_code == 6 { 
            let value1 = if modes[0] { vector[i as usize+1] } else { vector[vector[i as usize+1] as usize] };
            let value2 = if modes[1] { vector[i as usize+2] } else { vector[vector[i as usize+2] as usize] };
            //if (value1 != 0 && op_code == 5) || (value1 == 0 && op_code == 6) {
            if (value1 != 0) == (op_code == 5) {
                i = value2;
                continue
            }
            i += 3;
        }
        else if op_code == 7 { 
            let value1 = if modes[0] { vector[i as usize+1] } else { vector[vector[i as usize+1] as usize] };
            let value2 = if modes[1] { vector[i as usize+2] } else { vector[vector[i as usize+2] as usize] };

            let dest = vector[i as usize+3];
            vector[dest as usize] = if value1<value2 { 1 } else { 0 };
            i += 4;
        }
        else if op_code == 8 { 
            let value1 = if modes[0] { vector[i as usize+1] } else { vector[vector[i as usize+1] as usize] };
            let value2 = if modes[1] { vector[i as usize+2] } else { vector[vector[i as usize+2] as usize] };
            
            let dest = vector[i as usize+3];
            vector[dest as usize] = if value1==value2 { 1 } else { 0 };
            i += 4;
        }
        else if op_code == 99 {
            break
        }
        else {
            //println!("Unknown code: {}", op_code);
            break
        }
    }
    return output_vector
}

fn parse_file(file_name: &str) -> Vec<i32> {
    use std::fs;
    let file = fs::read_to_string(file_name).expect("Unable to read file");

    let mut vector: Vec<i32> = vec![];
    for value in file.split(",") {
        vector.push(value.parse().unwrap());
    }
    return vector
}

// #[derive(Debug, Eq, PartialEq)]
// struct Cube {
//     active: bool, neighbors: u8, x: i32, y: i32, z: i32
// }

// impl Cube {
//     fn new(x: i32, y: i32, z: i32) -> Cube {
//         return Cube { active: false, neighbors: 0, x: x, y: y, z: z }
//     }
// }