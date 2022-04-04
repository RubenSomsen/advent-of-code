pub fn run(file_location: &str) {
    println!("NOW RUNNING: {}", file!());

    let mut pair_vector = parse_file(file_location);
    
    use std::time::Instant;
    let now = Instant::now();

    let b = answer_b(&pair_vector);
    let a = answer_a(&mut pair_vector); // consumes vec

    let elapsed = now.elapsed();
    println!("PERFORMANCE: {:?}", elapsed);

    println!("ANSWER A: {:?}", a);
    println!("ANSWER B: {:?}", b);
}

fn answer_a(pair_vector: &mut Vec<Pair>) -> usize {
    let mut pair = pair_vector.pop().unwrap();
    while pair_vector.len() != 0 {
        pair = Pair::merge(pair, pair_vector.pop().unwrap());
        reduce(&mut pair);
    } 
    return pair.score()
}

fn answer_b(pair_vector: &Vec<Pair>) -> usize {
    let mut largest = 0;
    for i in 0..pair_vector.len()-1 {
        for ii in 1+i..pair_vector.len()-1 {
            for iii in 0..2 {
                let (a, b) = if iii==0 { (i, ii) } else { (ii, i) };
                let mut pair = Pair::merge(pair_vector[a].clone(), pair_vector[b].clone());
                reduce(&mut pair);
                let score = pair.score();
                if score>largest { largest = score }
            }
        }
    }
    return largest
}

fn reduce(pair: &mut Pair) {
    loop {
        loop {
            let e = explode(pair, 0, &mut 0);
            if e.is_none() { break } else { propagate(pair, e.unwrap(), &mut 0); }
        }
        if !split(pair) { break }
    }
}

fn explode(pair: &mut Pair, mut depth: u8, index: &mut u8) -> Option<Explosion> {
    depth += 1;
    if depth == 4 {
        for i in 0..2 {
            let p_ab = if i == 0 { &mut pair.pair_a } else { &mut pair.pair_b };
            if !p_ab.is_none() {
                let (val_a, val_b) = ((**p_ab).as_ref().unwrap().val_a, (**p_ab).as_ref().unwrap().val_b);
                **p_ab = None;
                return Some(Explosion { index: *index+1+i, val_a, val_b })
            }
        }
    }
    for i in 0..2 {
        let p_ab = if i == 0 { &mut pair.pair_a } else { &mut pair.pair_b };
        if p_ab.is_none() {
            *index += 1;
        }
        else {
            let e = explode((**p_ab).as_mut().unwrap(), depth, index);
            if !e.is_none() { return e }
        }
    }
    return None
}

// should theoretically be able to incorporate in explode function (fewer iterations)
fn propagate(pair: &mut Pair, e: Explosion, index: &mut u8) -> bool {
    for i in 0..2 {
        let (p_ab, v_ab) = if i == 0 { (&mut pair.pair_a, &mut pair.val_a) } 
                                                        else { (&mut pair.pair_b, &mut pair.val_b) };
        if p_ab.is_none() {
            *index += 1;
            if *index+1 == e.index { *v_ab += e.val_a }
            else if *index-1 == e.index  { *v_ab += e.val_b; return true }
        }
        else {
            if propagate((**p_ab).as_mut().unwrap(), e, index) { return true }
        }
    }
    return false
}

fn split(pair: &mut Pair) -> bool {    
    for i in 0..2 {
        let (p_ab, v_ab) = if i == 0 { (&mut pair.pair_a, &mut pair.val_a) } 
                                                        else { (&mut pair.pair_b, &mut pair.val_b) };
        if p_ab.is_none() {
            if *v_ab > 9 {
                *p_ab = Box::new(Some(Pair::split(*v_ab)));
                *v_ab = 0;
                return true
            }
        }
        else {
            if split((**p_ab).as_mut().unwrap()) { return true }
        }
    }
    return false
}


fn parse_file(file_name: &str) -> Vec<Pair> {
    use std::fs;
    let file = fs::read_to_string(file_name).expect("Unable to read file");
    let mut pair_vector = vec![];
    for line in file.lines() {
        pair_vector.push(parse_string(line, &mut 0));
    }
    pair_vector.reverse();
    return pair_vector
}

fn parse_string(string: &str, index: &mut usize) -> Pair {
    let mut pair = Pair::new();
    let mut pair_ab = &mut pair.pair_a;
    let mut val_ab = &mut pair.val_a;
    loop {
        *index += 1;
        let s = &string[*index..*index+1];
        if s == "[" {
            *pair_ab = Box::new(Some(parse_string(string, index)));
        }
        else if s == "]" {
            return pair
        }
        else if s == "," {
            pair_ab = &mut pair.pair_b;
            val_ab = &mut pair.val_b;
        }
        else { // int
            *val_ab = s.parse().unwrap();
        }
    }
} 

#[derive(Debug, Clone, Copy)]
struct Explosion { index: u8, val_a: u8, val_b: u8 } // what exploded and where

#[derive(Debug, Clone)]
struct Pair {
    val_a: u8, val_b: u8, pair_a: Box<Option<Pair>>, pair_b: Box<Option<Pair>>
}

impl Pair {
    fn new() -> Pair {
        return Pair { val_a: 0, val_b: 0, pair_a: Box::new(None), pair_b: Box::new(None) }
    }

    fn merge(pair_a: Pair, pair_b: Pair) -> Pair {
        return Pair { val_a: 0, val_b: 0, pair_a: Box::new(Some(pair_a)), pair_b: Box::new(Some(pair_b)) }
    }

    fn split(val: u8) -> Pair {
        return Pair { val_a: val/2, val_b: val/2 + val%2, pair_a: Box::new(None), pair_b: Box::new(None) }
    }

    fn score(&self) -> usize {
        let mut score = 0;
        for i in 0..2 {
            let (p_ab, v_ab) = if i == 0 { (&self.pair_a, &self.val_a) } 
                                                    else { (&self.pair_b, &self.val_b) };
            score += if p_ab.is_none() {
                *v_ab as usize*(3-i)
            }
            else {
                (**p_ab).as_ref().unwrap().score()*(3-i)
            };
        }
        return score
    }

    fn print(&self, is_first: bool) { // debug tool
        print!("[");
        for i in 0..2 {
            let (p_ab, v_ab) = if i == 0 { (&self.pair_a, &self.val_a) } 
                                                    else { (&self.pair_b, &self.val_b) };
            if p_ab.is_none() { print!("{}", v_ab); }
            else { (**p_ab).as_ref().unwrap().print(false); }
            if i == 0 { print!(",") }
        }
        print!("]");
        if is_first { println!(); }
    }
}