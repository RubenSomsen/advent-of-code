pub fn run(file_location: &str) {
    println!("NOW RUNNING: {}", file!());

    let mut vec = parse_file(file_location);
    
    use std::time::Instant;
    let now = Instant::now();

    let (a, vec_b) = answer_a(&mut vec);
    let b = answer_b(vec_b);

    let elapsed = now.elapsed();
    println!("PERFORMANCE: {:?}", elapsed);

    println!("ANSWER A: {:?}", a);
    println!("ANSWER B: {:?}", b);
}

fn answer_a(vec: &mut Vec<Vec<Brace>>) -> (usize, Vec<Vec<&mut Brace>>) {
    let mut counter = 0;
    let mut stacks = vec![];
    'a: for v in vec {
        let mut stack = vec![];
        for brace in v {
            if brace.open { stack.push(brace) }
            else { 
                let lifo = stack.pop().unwrap();
                if !lifo.compare(brace) {
                    counter += brace.score();
                    continue 'a
                }
            }
        }
        if stack.len() != 0 {
            stack.reverse();
            stacks.push(stack);
        }
    }
    return (counter, stacks)
}

fn answer_b(vec_b: Vec<Vec<&mut Brace>>) -> usize {
    let mut counters = vec![];
    for vec in vec_b {
        let mut counter = 0;
        for brace in vec {
            counter *=5;
            counter += brace.score_b();
        }
        counters.push(counter);
    }
    counters.sort();
    return counters[(counters.len()-1)/2]
}

fn parse_file(file_name: &str) -> Vec<Vec<Brace>> {
    use std::fs;
    let file = fs::read_to_string(file_name).expect("Unable to read file");
    let mut vec = vec![];
    for line in file.lines() {
        let mut vec2 = vec![];
        for c in line.chars() {
            let t = 
                     if c == '(' { (false,true,true) } 
                else if c == ')' { (false,true,false) }
                else if c == '{' { (true,true,true) }
                else if c == '}' { (true,true,false) }
                else if c == '[' { (false,false,true) }
                else if c == ']' { (false,false,false) }
                else if c == '<' { (true,false,true) }
                else if c == '>' { (true,false,false) }
                else { panic!("Illegal character in data set") };
                vec2.push(Brace::new(t));
        }
        vec.push(vec2);
    }
    return vec
}

struct Brace {
    pointy: bool, rounded: bool, open: bool
}

impl Brace {
    fn new((pointy, rounded, open): (bool, bool, bool)) -> Brace {
        return Brace { pointy, rounded, open }
    }

    fn score(&self) -> usize {
        let p = self.pointy;
        let r = self.rounded;
        return if p&&r { 1197 } else if !p&&r { 3 } else if p&&!r { 25137 } else { 57 }
    }

    fn score_b(&self) -> usize {
        let p = self.pointy;
        let r = self.rounded;
        return if p&&r { 3 } else if !p&&r { 1 } else if p&&!r { 4 } else { 2 }
    }

    fn compare(&self, brace: &Brace) -> bool {
        return self.pointy == brace.pointy && self.rounded == brace.rounded
    }
}