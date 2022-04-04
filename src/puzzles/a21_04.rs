pub fn run(file_location: &str) {
    println!("NOW RUNNING: {}", file!());

    let mut vecs = parse_file(file_location);
    
    use std::time::Instant;
    let now = Instant::now();

    let a = answer_a(&mut vecs);
    let b = answer_b(&mut vecs);

    let elapsed = now.elapsed();
    println!("PERFORMANCE: {:?}", elapsed);

    println!("ANSWER A: {:?}", a);
    println!("ANSWER B: {:?}", b);
}

fn answer_a(vecs: &mut (Vec<u8>, Vec<Vec<(usize, usize, usize)>>, Vec<Board>)) -> u32 {
    let (draw_vec, loc_vec, board_vec) = vecs;
    for i in 0..draw_vec.len() {
        let val = draw_vec[i];
        let vec = &loc_vec[val as usize];
        for ii in 0..vec.len() {
            let (b, x, y) = vec[ii];
            board_vec[b].set(x,y);
            if board_vec[b].check(x,y) {
                return val as u32*board_vec[b].count()
            }
        }
    }
    return 0
}

fn answer_b(vecs: &mut (Vec<u8>, Vec<Vec<(usize, usize, usize)>>, Vec<Board>)) -> u32 {
    let (draw_vec, loc_vec, board_vec) = vecs;
    let mut check_vec = vec![false; board_vec.len()]; //
    let mut counter = board_vec.len(); //
    for i in 0..draw_vec.len() {
        let val = draw_vec[i];
        let vec = &loc_vec[val as usize];
        for ii in 0..vec.len() {
            let (b, x, y) = vec[ii];
            board_vec[b].set(x,y);
            if !check_vec[b] && board_vec[b].check(x,y) { // .
                check_vec[b] = true; //
                counter -= 1; //
            }
            if counter == 0 { //
                return val as u32*board_vec[b].count()
            }
        }
    }
    return 0
}

fn parse_file(file_name: &str) -> (Vec<u8>, Vec<Vec<(usize, usize, usize)>>, Vec<Board>) {
    use std::fs;
    let file = fs::read_to_string(file_name).expect("Unable to read file");

    let mut draw_vec = vec![];
    let mut loc_vec = vec![vec![]; 100];
    let mut board_vec = vec![];

    let mut parts = file.split("\n\n");
    draw_vec = parts.next().unwrap().split(",").map(|v| v.parse().unwrap()).collect();

    for part in parts {
        let mut board = Board::new();
        for (y, line) in part.lines().enumerate() {
            for (x, val) in line.split_whitespace().enumerate() {
                let val = val.parse().unwrap();
                board.set_val(x, y, val);
                loc_vec[val as usize].push((board_vec.len(), x, y));
            }
        }
        board_vec.push(board);
    }
    return (draw_vec, loc_vec, board_vec)
}

#[derive(Debug, Eq, PartialEq)]
struct Board {
    state: Vec<(u8, bool)>
}

impl Board {

    fn new() -> Board {
        return Board { state: vec![(0, false); 5*5] }
    }
    
    fn get(&self, x: usize, y: usize) -> bool {
        if x>=5 || y>=5 { panic!("board overflow") }
        return self.state[x+y*5].1
    }

    fn set(&mut self, x: usize, y: usize) {
        if x>=5 || y>=5 { panic!("board overflow") }
        self.state[x+y*5].1 = true;
    }

    fn set_val(&mut self, x: usize, y: usize, val: u8) {
        if x>=5 || y>=5 { panic!("board overflow") }
        self.state[x+y*5].0 = val;
    }

    fn check(&self, x: usize, y: usize) -> bool {
        if x>=5 || y>=5 { panic!("board overflow") }
        let (mut hori, mut vert) = (true, true);
        for i in 0..5 {
            if hori { hori = hori & self.get((x+i)%5,y); }
            if vert { vert = vert & self.get(x, (y+i)%5); }
            if !hori && !vert { return false }
        }
        return true
    }

    fn count(&mut self) -> u32 {
        let mut counter = 0;
        for i in 0..self.state.len() {
            if !self.state[i].1 { counter += self.state[i].0 as u32; }
        }
        return counter
    }
}