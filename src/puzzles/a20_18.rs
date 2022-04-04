pub fn run(file_location: &str) {
    println!("NOW RUNNING: {}", file!());
    
    use std::time::Instant;
    let now = Instant::now();

    let a: u64 = answer(&mut parse_file(file_location, false));
    let b: u64 = answer(&mut parse_file(file_location, true));

    let elapsed = now.elapsed();
    println!("PERFORMANCE: {:?}", elapsed);

    println!("ANSWER A: {}", a);
    println!("ANSWER B: {}", b);
}

fn answer(vector: &mut Vec<Vec<Calc>>) -> u64 {
    let mut answer = 0;
    for v in vector {
        answer += calculate_sum(v);
    }
    return answer
}

fn calculate_sum(sum: &Vec<Calc>) -> u64 {
    let mut answer: u64 = 0;
    for calc in sum {
        let mut amount = calc.val;
        if amount == 0 { amount = calculate_sum(&calc.vector); }
        if calc.add { answer += amount }
        else { answer *= amount }
    }
    return answer
}

fn parse_file(file_name: &str, add_first: bool) -> Vec<Vec<Calc>> {
    use std::fs;
    let file = fs::read_to_string(file_name).expect("Unable to read file");

    let mut vector:  Vec<Vec<Calc>> = vec![];
   
    for line in file.lines() {
        vector.push(parse_math(line, add_first));
    }
    return vector
}

fn parse_math(string: &str, add_first: bool) -> Vec<Calc> {
    let mut vector: Vec<Calc> = vec![];
    let mut skipping = 0;
    let mut skipped_from = 0;
    let mut calc: Calc = Calc::new();
    for (i, c) in string.chars().enumerate() {
        if c == ' ' { continue }
        else if c == '+' && skipping<1 { calc.add = true; }
        else if c == '*' && skipping<1 { calc.add = false; }
        else if c == '(' { 
            if skipping == 0 { skipped_from = i; }
            skipping += 1;
        }
        else { 
            if c == ')' { 
                skipping -= 1;
                if skipping<1 { 
                    calc.vector = parse_math(&string[skipped_from+1..i], add_first);
                }
            }
            else if skipping<1 { 
                calc.val = c as u64 - '0' as u64;
            }
            if skipping<1 {
                if add_first && calc.add && vector.len()>0 {
                    calc = vector.pop().unwrap().merge(calc);
                }
                vector.push(calc);
                calc = Calc::new();
            }
        }
    }
    return vector
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Calc {
    add: bool, val: u64, vector: Vec<Calc>
}

impl Calc {
    fn new() -> Calc {
        return Calc { add: true, val: 0, vector: vec![] }
    }

    // merges two calcs, i.e. they'll be in brackets, so +1 *2 +5 becomes +1 *(+2 +5)
    fn merge(mut self, calc: Calc) -> Calc {
        let mut clone = self.clone();
        clone.add = true;
        if self.val != 0 {
            self.vector.push(clone);
            self.val = 0;
        }
        else {
            self.vector = vec![clone];
        }
        self.vector.push(calc);
        return self
    }
}