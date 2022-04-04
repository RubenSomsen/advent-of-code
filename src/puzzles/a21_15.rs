use std::collections::BinaryHeap;

pub fn run(file_location: &str) {
    println!("NOW RUNNING: {}", file!());

    let mut grid = parse_file(file_location);
    let mut grid_5x = get_new_5x_grid(&grid);

    use std::time::Instant;
    let now = Instant::now();

    let a = a_star(&mut grid);
    let b = a_star(&mut grid_5x);

    let elapsed = now.elapsed();
    println!("PERFORMANCE: {:?}", elapsed);

    println!("ANSWER A: {:?}", a);
    println!("ANSWER B: {:?}", b);
}

// A* distance search algorithm with binary heap
// Guaranteed to find the best answer (no shortcuts)
//
// Note that values can enter the heap multiple times
// (when distance is updated), but will be skipped if
// popped twice. This is because it's inefficient to
// update the distance of values inside the heap, so
// duplicates + skipping is an acceptable alternative.

fn a_star(grid: &mut Grid) -> u64 {
    let mut sorted_dist_heap = BinaryHeap::new();
    sorted_dist_heap.push((0,0,0)); // the heap is sorted on the first value in the tuple
    loop {
        let (_, x, y) = sorted_dist_heap.pop().unwrap(); // get x,y of target value
        if grid.is_touched(x,y) { continue } else { grid.touch(x,y) } // skip if touched, else touch
        if x+y==(grid.size-1)*2 { return grid.get_dist(x,y) } // if goal, return final distance
        let neighbors = grid.get_neighbors(x,y);
        for (nx,ny) in neighbors {
            let n_dist = grid.get(nx,ny) as u64 + grid.get_dist(x,y); // neighbor distance
            let is_shorter_path = n_dist < grid.get_dist(nx, ny);
            if !grid.is_touched(nx,ny) && is_shorter_path { // if neighbor is relevant
                grid.set_dist(nx,ny, n_dist); // update/set its distance
                let n_weight = (grid.size as u64*2-2-((nx+ny) as u64))*1; // neighbor weight for A*, though it barely matters
                sorted_dist_heap.push((u64::MAX-(n_dist+n_weight),nx,ny)); // push neighbor, sorted to pop lowest distance
            }
        }
    }
}

// creates a square grid that can be called by x, y coordinates
fn parse_file(file_name: &str) -> Grid {
    use std::fs;
    let file = fs::read_to_string(file_name).expect("Unable to read file");
    let mut grid = Grid::new(file.lines().next().unwrap().len());
    for (y, line) in file.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let val = c as u16 - 48;
            grid.set(x as u16, y as u16, val);
        }
    }
    return grid
}

// takes an existing grid, creates a new 5x larger one, while updating the cost
fn get_new_5x_grid(grid: &Grid) -> Grid {
    let mut new_grid = Grid::new(grid.size as usize*5);
    for ry in 0..5 {
        for rx in 0..5 {
            for y in 0..grid.size {
                for x in 0..grid.size {
                    new_grid.set(x+grid.size*rx,y+grid.size*ry, (grid.get(x,y)+rx+ry-1)%9+1);
                }
            }
        }
    }
    return new_grid
}

struct Grid {
    state: Vec<Value>, size: u16
}

impl Grid {
    fn new(size: usize) -> Grid {
        let mut grid = Grid { state: vec![Value::new(); size*size], size: size as u16 };
        grid.state[0].distance = 0;
        return grid
    }

    fn set(&mut self, x: u16, y: u16, cost: u16) {
        self.state[x as usize+y as usize*self.size as usize].cost = cost;
    }

    fn get(&self, x: u16, y: u16) -> u16 {
        return self.state[x as usize+y as usize*self.size as usize].cost
    }

    fn touch(&mut self, x: u16, y: u16) {
        self.state[x as usize+y as usize*self.size as usize].is_touched = true;
    }

    fn is_touched(&self, x: u16, y: u16) -> bool { // i.e. whether it has been visited
        return self.state[x as usize+y as usize*self.size as usize].is_touched
    }

    fn set_dist(&mut self, x: u16, y: u16, distance: u64) {
        self.state[x as usize+y as usize*self.size as usize].distance = distance;
    }

    fn get_dist(&self, x: u16, y: u16) -> u64 {
        return self.state[x as usize+y as usize*self.size as usize].distance
    }

    fn print(&self, size: usize) { // size 0 = full grid
        for (i, val) in self.state.iter().enumerate() {
            if size != 0 && (i%self.size as usize>size-1 || i>self.size as usize*size-1) { continue }
            if i%self.size as usize == 0 { print!("{}", "\n") }
            print!("{}", val.cost as u16);
        }
        println!();
    }

    fn get_neighbors(&self, x: u16, y: u16) -> Vec<(u16, u16)> {
        let mut vec = vec![];
        if x != 0 { vec.push((x-1,y)); }
        if y != 0 { vec.push((x,y-1)); }
        if x != self.size-1 { vec.push((x+1,y)); }
        if y != self.size-1 { vec.push((x,y+1)); }
        return vec
    }
}

#[derive(Debug, Clone, Copy)]
struct Value {
    cost: u16, distance: u64, is_touched: bool
}

impl Value {
    fn new() -> Value {
        return Value { cost: 0, distance: u64::MAX, is_touched: false }
    }
}