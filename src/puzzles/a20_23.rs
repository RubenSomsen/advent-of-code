const SEQUENCE: [u64; 9] = [5,8,3,9,7,6,2,4,1];
//const SEQUENCE: [u64; 9] = [3,8,9,1,2,5,4,6,7]; // practice set
const SLEN: u64 = 1000*1000;// either SEQUENCE.len() as u64 for A, or 1000*1000 for B

pub fn run(_file_location: &str) {
    println!("NOW RUNNING: {}", file!());

    let mut sequence = Seq::new();
    
    use std::time::Instant;
    let now = Instant::now();

    let ab = answer(&mut sequence, 10*1000*1000);// 100 for A, 10*1000*1000 for B

    let elapsed = now.elapsed();
    println!("PERFORMANCE: {:?}", elapsed);

    println!("ANSWER: {:?}", ab);
}

fn answer(sequence: &mut Seq, turns: usize) -> u64 {
    for i  in 0..turns {
        sequence.move_cups();
    }
    let a1 = sequence.next_loc(1, 1);
    let a2 = sequence.next_loc(a1, 1);
    return a1*a2
}

fn create_sequence() -> Vec<u64> {
    let mut neighbor_vec: Vec<u64> = vec![0; 1+SLEN as usize];
    for i in 0..SLEN as usize {
        if i<SEQUENCE.len() {
            neighbor_vec[SEQUENCE[i] as usize] = SEQUENCE[(i+1)%SEQUENCE.len()] as u64;
        }
        else {
            neighbor_vec[1+i] = 2+i as u64;
        }
    }
    neighbor_vec[0] = SEQUENCE[0]; // starting pos
    if SLEN as usize>SEQUENCE.len() { // i.e. if more numbers were added
        neighbor_vec[SLEN as usize] = SEQUENCE[0]; // link beginning SEQ to rest
        neighbor_vec[SEQUENCE[SEQUENCE.len()-1] as usize] = 1+SEQUENCE.len() as u64; // link end SEQ to rest
    }
    return neighbor_vec
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Seq {
    numbers: Vec<u64>
}

impl Seq {
    fn new() -> Seq {
        return Seq { numbers: create_sequence() }
    }

    fn get_pos(&self) -> u64 {
        return self.numbers[0]
    }

    fn set_next_pos(&mut self) {
        self.numbers[0] = self.numbers[self.numbers[0] as usize];
    }

    fn next_loc(&self, mut loc: u64, moves: u8) -> u64 {
        for _ in 0..moves {
            loc = self.numbers[loc as usize];
        }
        return loc
    }

    fn get_prev_nr(&self) -> u64 {
        let mut pos = self.get_pos();
        let p1 = self.next_loc(pos, 1);
        let p2 = self.next_loc(p1, 1);
        let p3 = self.next_loc(p2, 1);
        loop {
            pos = ((SLEN+pos-2)%SLEN)+1;
            if p1 != pos && p2 != pos && p3 != pos {
                return pos
            }
        }
    }

    fn move_cups(&mut self) {
        let cup_a = self.get_pos();
        let cup_b = self.next_loc(cup_a, 3);
        let cup_c = self.get_prev_nr();
        let loc_a = self.next_loc(cup_a, 1);
        let loc_b = self.next_loc(cup_b, 1);
        let loc_c = self.next_loc(cup_c, 1);
        self.numbers[cup_a as usize] = loc_b;
        self.numbers[cup_b as usize] = loc_c;
        self.numbers[cup_c as usize] = loc_a;
        self.set_next_pos();
    }
}