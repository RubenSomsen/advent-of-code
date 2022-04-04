pub fn run(file_location: &str) {
    println!("NOW RUNNING: {}", file!());

    let mut vec = parse_file(file_location);
    
    use std::time::Instant;
    let now = Instant::now();

    let a = answer_a(&mut vec);
    let b = answer_b(&mut vec);

    let elapsed = now.elapsed();
    println!("PERFORMANCE: {:?}", elapsed);

    println!("ANSWER A: {:?}", a);
    println!("ANSWER B: {:?}", b);
}

fn answer_a(vec: &mut Vec<Sequence>) -> usize {
    let mut counter = 0;
    for seq in vec {
        for i in 0..seq.ten.len() {
            let c = seq.ten[i].set_count;
            if c==2 { seq.set_value(1, i); }
            else if c==3 { seq.set_value(7, i); } 
            else if c==4 { seq.set_value(4, i); } 
            else if c==7 { seq.set_value(8, i); } 
        }
        for i in 0..4 {
            let v = seq.ten[seq.four[i]].value;
            if v==1||v==4||v==7||v==8 { counter += 1 }
        }
    }
    return counter
}

fn answer_b(vec: &mut Vec<Sequence>) -> usize {
    let mut counter = 0;
    for seq in vec {
        let v = [(4,9,1,6,0),(1,3,4,2,5)];
        for i in 0..2 {
            let vec56 = seq.get56(i==1);
            for six in vec56 {
                if seq.ten[six].envelops(false, seq.ten[seq.values[v[i].0]]) {
                    seq.set_value(v[i].1, six);
                }
                else if seq.ten[six].envelops(true, seq.ten[seq.values[v[i].2]]) {
                    seq.set_value(v[i].3, six);
                }
                else { 
                    seq.set_value(v[i].4, six); 
                }
            }
        }
        for (i, f) in seq.four.iter().enumerate() {
            counter += seq.ten[*f].value*usize::pow(10, 3-i as u32);
        }
    }
    return counter
}

fn parse_file(file_name: &str) -> Vec<Sequence> {
    use std::fs;
    let file = fs::read_to_string(file_name).expect("Unable to read file");
    let mut vec = vec![];

    for line in file.lines() {
        let mut sequence = Sequence::new();
        let mut line_split = line.split(" | ");
        let ten: Vec<&str> = line_split.next().unwrap().split_whitespace().collect();
        let four: Vec<&str> = line_split.next().unwrap().split_whitespace().collect();
        for (i, t) in ten.iter().enumerate() {
            for c in t.chars() {
                sequence.ten[i].set(c as usize-97);
            }
        }
        for (i, t) in four.iter().enumerate() {
            let mut digit = Digit::new();
            for c in t.chars() {
                digit.set(c as usize-97);
            }
            sequence.four[i] = sequence.compare(digit);
        }
        vec.push(sequence);
    }    
    return vec
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
struct Digit {
    state: [bool; 7], set_count: u8, value: usize
}

impl Digit {
    fn new() -> Digit {
        return Digit { state: [false; 7], set_count: 0, value: 10 }
    }

    fn set(&mut self, location: usize) {
        self.state[location] = true;
        self.set_count += 1;
    }

    fn envelops(&self, inverse: bool, digit: Digit) -> bool {
        for i in 0..7 {
            if (digit.state[i] == !inverse) && (self.state[i] != true) { return false }
        }
        return true
    }
}

struct Sequence {
    ten: [Digit; 10], values: [usize; 10], four: [usize; 4]
}

impl Sequence {
    fn new() -> Sequence {
        return Sequence { ten: [Digit::new(); 10], values: [10; 10], four: [10; 4] }
    }

    fn compare(&self, d1: Digit) -> usize {
        'a: for i1 in 0..10 {
            let d2 = &self.ten[i1];
            if d1.set_count != d2.set_count { continue }
            for i2 in 0..7 {
                if d1.state[i2] != d2.state[i2] { continue 'a }
            }
            return i1
        }
        panic!("Should have already ended");
    }

    fn get56(&mut self, is_five: bool) -> Vec<usize> {
        let mut vec = vec![];
        let five = if is_five {5} else {6};
        for (i, digit) in self.ten.iter().enumerate() {
            if digit.set_count == five {
                vec.push(i);
            }
        }
        return vec
    }

    fn set_value(&mut self, value: usize, ten_index: usize) {
        self.values[value] = ten_index;
        self.ten[ten_index].value = value;
    }
}