pub fn run(file_location: &str) {
    println!("NOW RUNNING: {}", file!());

    let mut vector: Vec<(char, u16)> = parse_file(file_location);

    use std::time::Instant;
    let now = Instant::now();

    let answer_a: i32 = answer_a(&mut vector);
    let answer_b: i32 = answer_b(&mut vector);

    let elapsed = now.elapsed();
    println!("PERFORMANCE: {:?}", elapsed);

    println!("ANSWER A: {}", answer_a);
    println!("ANSWER B: {}", answer_b);

    assert_eq!(turn('N', 180, true), 'S');
    assert_eq!(turn('E', 180, true), 'W');
    assert_eq!(turn('S', 270, false), 'W');
    assert_eq!(turn('W', 360, false), 'W');
}

fn answer_b(instruction_vector: &mut Vec<(char, u16)>) -> i32 {
    let mut pos = Pos { x: 0, y:0, direction:'E' };
    let mut wp = Relative { x:10, y:1 };
    for instruction in instruction_vector {
        let (ins, amt) = (instruction.0, instruction.1); // instruction, amount
        if ins == 'N' || ins == 'E' || ins == 'S' || ins == 'W' { 
            let rel = go(ins, amt);
            wp.x += rel.x;
            wp.y += rel.y;
        }
        else if ins == 'L' || ins == 'R' { 
            for _ in 0..amt%89 {
                if ins == 'R' { wp.x *= -1 } else { wp.y *= -1; }
                let tmp = wp.x;
                wp.x = wp.y;
                wp.y = tmp;
            }
        }
        else { 
            pos.x += wp.x*(amt as i32);
            pos.y += wp.y*(amt as i32);
        }
    }
    return pos.x.abs() + pos.y.abs()
}

fn answer_a(instruction_vector: &mut Vec<(char, u16)>) -> i32 {
    let mut pos = Pos { x: 0, y:0, direction:'E' };
    for instruction in instruction_vector {
        let (ins, amt) = (instruction.0, instruction.1);
        if ins == 'N' || ins == 'E' || ins == 'S' || ins == 'W' { 
            let rel = go(ins, amt);
            pos.x += rel.x;
            pos.y += rel.y; 
        }
        else if ins == 'L' || ins == 'R' { pos.direction = turn(pos.direction, amt, ins == 'R') }
        else { 
            let rel = go(pos.direction, amt);
            pos.x += rel.x;
            pos.y += rel.y; 
        }
    }
    return pos.x.abs() + pos.y.abs()
}

fn go(direction: char, amount: u16) -> Relative {
    let mut rel = Relative { x:0, y:0 };
         if direction == 'N' { rel.y += amount as i32 }
    else if direction == 'E' { rel.x += amount as i32 }
    else if direction == 'S' { rel.y -= amount as i32 }
    else if direction == 'W' { rel.x -= amount as i32 }
    return rel
}


fn turn(mut c: char, deg: u16, right: bool) -> char {
    for _ in 0..deg%89 { 
             if c == 'N' { c = if right { 'E' } else { 'W' } }
        else if c == 'E' { c = if right { 'S' } else { 'N' } }
        else if c == 'S' { c = if right { 'W' } else { 'E' } }
        else if c == 'W' { c = if right { 'N' } else { 'S' } }
    }
    return c
}

fn parse_file(file_name: &str) -> Vec<(char, u16)> {
    use std::fs;
    let file = fs::read_to_string(file_name).expect("Unable to read file");

    let mut instruction_vector: Vec<(char, u16)> = vec![];
    for instruction in file.lines() {
        let c: char = instruction[0..1].parse().unwrap();
        let i: u16 = instruction[1..].parse().unwrap();
        instruction_vector.push((c, i));
    }
    return instruction_vector
}

struct Pos {
    x: i32, y: i32, direction: char
}

struct Relative {
    x: i32, y: i32
}