const LEN: usize = 1000; // hard coded limitation

pub fn run(file_location: &str) {
    println!("NOW RUNNING: {}", file!());

    let mut directions = parse_file(file_location);
    
    use std::time::Instant;
    let now = Instant::now();

    let (a, hex) = answer_a(&mut directions);
    let b = answer_b(hex, 100);

    let elapsed = now.elapsed();
    println!("PERFORMANCE: {:?}", elapsed);

    println!("ANSWER A: {:?}", a);
    println!("ANSWER B: {:?}", b);
}

fn answer_b(mut hex: Hex, turns: usize) -> u16 {
    for _ in 0..turns {
        let mut changes = vec![];
        for (y, x_array) in hex.grid.iter().enumerate() {
            for (x, b) in x_array.iter().enumerate() {
                if *b { // if black
                    let whites = hex.get_white_neighbors((x,y)); // get white neighbors
                    if whites.len()<4 || whites.len() == 6 { changes.push((x,y)); } // if 3+ or 0 black, go white
                    for white in whites {
                        let whites2 = hex.get_white_neighbors(white); // get white neighbors
                        if whites2.len()==4 { changes.push(white); } // if exactly 2 black, go black
                    }
                }
            } 
        }
        changes.sort(); // sort of a lazy way of getting rid of doubles, but oh well
        let mut last_change = (LEN,LEN);
        for change in changes { 
            if change == last_change { continue }
            let b = hex.flip_tile(change);
            last_change = change;
        }
    }
    return hex.count
}

fn answer_a(directions: &mut Vec<Vec<(i8,i8)>>) -> (u16, Hex) {
    let mut hex = Hex::new();
    for coords in directions {
        let mut xy = (0,0);
        for co in coords {
            xy = Hex::get_new_xy(xy, co);
        }
        hex.flip_tile(xy);
    }
    return (hex.count, hex)
}

fn parse_file(file_name: &str) -> Vec<Vec<(i8,i8)>> {
    use std::fs;
    let file = fs::read_to_string(file_name).expect("Unable to read file");

    let mut directions = vec![];
    for line in file.lines() {
        let mut coords = vec![];
        let mut chars = line.chars();
        loop {
            let get_c = chars.next();
            if get_c == None { break }
            let mut c = get_c.unwrap();
            let mut offset = (1,-1); // assume E
            if c == 'n' || c == 's' {
                if c == 'n' { offset = (0,-1); }
                else { offset = (1,0); }
                c = chars.next().unwrap();
            }
            if c == 'w' { // swap if W
                offset = (offset.1, offset.0);
            }
            coords.push(offset);
        }
        directions.push(coords);
    }
    return directions
}

#[derive(Debug, Eq, PartialEq)]
struct Hex {
    grid: Vec<Vec<bool>>, count: u16
}

impl Hex {
    fn new() -> Hex {
        return Hex { grid: vec![vec![false; LEN]; LEN], count: 0 }
    }

    fn flip_tile(&mut self, xy: (usize, usize)) -> bool {
        self.grid[xy.1][xy.0] = !self.grid[xy.1][xy.0];
        if self.grid[xy.1][xy.0] { self.count += 1; }
        else { self.count -= 1; }
        return self.grid[xy.1][xy.0]
    }

    fn get_new_xy(xy: (usize, usize), offset: &(i8, i8)) -> (usize, usize) {
        let x = ((LEN as i16 + xy.0 as i16 + offset.0 as i16)%LEN as i16) as usize;
        let y = ((LEN as i16 + xy.1 as i16 + offset.1 as i16)%LEN as i16) as usize;
        return (x, y)
    }

    fn get_white_neighbors(&self, xy: (usize, usize)) -> Vec<(usize,usize)> {
        let offsets = [(1,0),(0,1),(-1,1),(1,-1),(-1,0),(0,-1)];
        let mut whites = vec![];
        for offset in offsets.iter() {
            let n_xy = Hex::get_new_xy(xy, offset);
            if !self.grid[n_xy.1][n_xy.0] {
                whites.push(n_xy);
            }
        }
        return whites
    }
}