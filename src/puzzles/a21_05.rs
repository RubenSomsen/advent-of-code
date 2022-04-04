pub fn run(file_location: &str) {
    println!("NOW RUNNING: {}", file!());

    let mut vec = parse_file(file_location);
    
    use std::time::Instant;
    let now = Instant::now();

    let a = answer(&mut vec, false);
    let b = answer(&mut vec, true);

    let elapsed = now.elapsed();
    println!("PERFORMANCE: {:?}", elapsed);

    println!("ANSWER A: {:?}", a);
    println!("ANSWER B: {:?}", b);
}

fn answer(vec: &mut Vec<Line>, is_b: bool) -> u32 {
    let mut counter = 0;
    let mut grid = Grid::new();
    for line in vec {
        let lineget = line.get(is_b);
        for xy in lineget {
            if grid.set(xy.0, xy.1) == 2 { counter += 1; }
        }
    }
    return counter
}

fn parse_file(file_name: &str) -> Vec<Line> {
    use std::fs;
    let file = fs::read_to_string(file_name).expect("Unable to read file");

    let mut line_vec = vec![];

    for line in file.lines() {
       let mut line_split = line.split(" -> ");
       let xya = line_split.next().unwrap();
       let xyb = line_split.next().unwrap();
       let mut xya_split = xya.split(",");
       let xa = xya_split.next().unwrap().parse().unwrap();
       let ya = xya_split.next().unwrap().parse().unwrap();
       let mut xyb_split = xyb.split(",");
       let xb = xyb_split.next().unwrap().parse().unwrap();
       let yb = xyb_split.next().unwrap().parse().unwrap();
       line_vec.push(Line::new(xa, xb, ya, yb));
    }
    return line_vec
}

#[derive(Debug, Eq, PartialEq)]
struct Line {
    xa: usize, xb: usize, ya: usize, yb: usize
}

impl Line {

    fn new(xa: usize, xb: usize, ya: usize, yb: usize) -> Line {
        return Line { xa, xb, ya, yb }
    }

    fn get(&self, is_b: bool) -> Vec<(usize, usize)> {
        let s = self;
        if !is_b && s.xa!=s.xb && s.ya!=s.yb { return vec![] }
        let (mut x, mut y) = (s.xa, s.ya);
        let mut vec = vec![(x,y)];
        while x != s.xb || y != s.yb {
            if s.xa!=s.xb { if s.xb>s.xa { x += 1 } else { x -= 1 } }
            if s.ya!=s.yb { if s.yb>s.ya { y += 1 } else { y -= 1 } }
            vec.push((x,y));
        }
        return vec
    }
}

struct Grid {
    state: Vec<usize>
}

impl Grid {
    fn new() -> Grid {
        return Grid { state: vec![0; 1000*1000] }
    }

    fn set(&mut self, x: usize, y: usize) -> usize {
        self.state[x+y*1000] += 1;
        return self.state[x+y*1000]
    }
}
