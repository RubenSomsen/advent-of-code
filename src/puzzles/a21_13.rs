pub fn run(file_location: &str) {
    println!("NOW RUNNING: {}", file!());

    let (mut grid, fold_vec) = parse_file(file_location);
    
    use std::time::Instant;
    let now = Instant::now();

    let a = answer(&mut grid, &fold_vec);
    //let b = 0; //answer_b(&mut vec);

    let elapsed = now.elapsed();
    println!("PERFORMANCE: {:?}", elapsed);

    println!("ANSWER A: {:?}", a);
    println!("ANSWER B: {:?}", "HECRZKPR (see print)");
}

fn answer(grid: &mut Grid, fold_vec: &Vec<bool>) -> usize {
    let mut count = 0;
    for i in 0..fold_vec.len() {
        grid.fold(fold_vec[i]);
        if i == 0 { count = grid.count() };
    }
    grid.print();
    return count
}

fn parse_file(file_name: &str) -> (Grid, Vec<bool>) {
    use std::fs;
    let file = fs::read_to_string(file_name).expect("Unable to read file");
    let mut grid = Grid::new();
    let mut fold_vec = vec![];
    let mut split = file.split("\r\n\r\n");
    let first = split.next().unwrap();
    let second = split.next().unwrap();
    for (i, line) in second.lines().enumerate() {
        let is_x_fold = &line[11..12] == "x";
        
        if i<2 { // lazily assumes the first two entries are NOT both x or y
            let h_w = if is_x_fold { &mut grid.width } else { &mut grid.height };
            *h_w = line[13..].parse().unwrap();
            *h_w = *h_w*2+1; // not sure why this needs two lines
        }
        fold_vec.push(is_x_fold);
    }
    grid.state = vec![false; grid.height*grid.width];
    grid.v_height = grid.height;
    grid.v_width = grid.width;

    for line in first.lines() {
        let mut split = line.split(",");
        let (x, y) = (split.next().unwrap().parse().unwrap(), split.next().unwrap().parse().unwrap());
        grid.set(x, y);
    }
    return (grid, fold_vec)
}

struct Grid {
    height: usize, width: usize, v_height: usize, v_width: usize, state: Vec<bool>
}

impl Grid {
    fn new() -> Grid {
        return Grid { height: 0, width: 0, v_height: 0, v_width: 0, state: vec![] }
    }

    fn get(&self, x: usize, y: usize) -> bool {
        return self.state[x+self.width*y]
    }

    fn set(&mut self, x: usize, y: usize) {
        self.state[x+self.width*y] = true;
    }

    fn fold(&mut self, is_x_fold: bool) {
        if !is_x_fold {
            self.v_height = (self.v_height-1)/2;
            for y in 0..self.v_height {
                for x in 0..self.v_width {
                    let (x2, y2) = (x, self.v_height*2-y);
                    if self.get(x2, y2) { self.set(x,y) } 
                }
            }
        }
        else {
            self.v_width = (self.v_width-1)/2;
            for y in 0..self.v_height {
                for x in 0..self.v_width {
                    let (x2, y2) = (self.v_width*2-x, y);
                    if self.get(x2, y2) { self.set(x,y) } 
                }
            }
        }
    }

    fn count(&self) -> usize {
        let mut counter = 0;
        for y in 0..self.v_height {
            for x in 0..self.v_width {
                counter += self.get(x,y) as usize;
            }
        }
        return counter
    }

    fn print(&self) {
        for y in 0..self.v_height {
            for x in 0..self.v_width {
                let c = if self.get(x,y) { '#' } else { ' ' };
                print!("{}",c);
            }
            println!();
        }
        println!();
    }
}