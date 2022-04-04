pub fn run(file_location: &str) {
    println!("NOW RUNNING: {}", file!());

    let mut grid = parse_file(file_location);
    
    use std::time::Instant;
    let now = Instant::now();

    let (a, valleys) = answer_a(&mut grid);
    let b = answer_b(&mut grid, valleys);

    let elapsed = now.elapsed();
    println!("PERFORMANCE: {:?}", elapsed);

    println!("ANSWER A: {:?}", a);
    println!("ANSWER B: {:?}", b);
}

fn answer_a(grid: &mut Grid) -> (usize, Vec<(usize,usize)>) {
    let mut counter = 0;
    let mut valleys = vec![];
    for y in 0..grid.height {
        'a: for x in 0..grid.width {
            let tile = grid.get_im(x,y);
            if tile.known { continue }
            let grid_neighbors = grid.neighbors(x,y);
            for xy in grid_neighbors {
                if tile.height>grid.get_im(xy.0, xy.1).height {
                    grid.get(x,y).known = true;
                    continue 'a
                }
            }
            counter += tile.height as usize+1;
            grid.get(x,y).known = true;
            grid.get(x,y).valley = true;
            valleys.push((x,y));
        }
    }
    return (counter, valleys)
}

fn answer_b(grid: &mut Grid, valleys: Vec<(usize,usize)>) -> usize {
    let mut counters = vec![];
    for xy in valleys {
        counters.push(count_basin(grid, &vec![xy]));
    }
    counters.sort();
    counters.reverse();
    return counters[0]*counters[1]*counters[2]
}

fn count_basin(grid: &mut Grid, coords: &Vec<(usize,usize)>) -> usize {
    if coords.len() == 0 { return 0 }
    let mut neighbors = vec![];
    for (x,y) in coords {
        grid.get(*x,*y).basin = true;
        for (x, y) in grid.neighbors(*x,*y) {
            let tile = grid.get(x,y);
            if tile.height != 9 && tile.basin != true {
                tile.basin = true;
                neighbors.push((x,y));
            }
        }
    }
    return coords.len() + count_basin(grid, &neighbors)
} 

fn parse_file(file_name: &str) -> Grid {
    use std::fs;
    let file = fs::read_to_string(file_name).expect("Unable to read file");
    let mut grid = Grid::new();
    for line in file.lines() {
        if grid.width == 0 { grid.width = line.len() }
        grid.height += 1;
        let mut vec: Vec<Tile> = line.chars().map(|c| Tile::new(c as u8 - 48)).collect();
        grid.state.append(&mut vec);
    }
    return grid
}

struct Grid {
    state: Vec<Tile>, width: usize, height: usize
}

impl Grid {
    fn new() -> Grid {
        return Grid { state: vec![], width: 0, height: 0 }
    }

    fn get(&mut self, x: usize, y: usize) -> &mut Tile {
        if x>=self.width || y>=self.height { panic!("xy out of bounds!") }
        return &mut self.state[x+(y*self.width)]
    }

    fn get_im(&self, x: usize, y: usize) -> &Tile {
        if x>=self.width || y>=self.height { panic!("xy out of bounds!") }
        return &self.state[x+(y*self.width)]
    }

    fn neighbors(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut vec = vec![];
        if x != 0 { vec.push((x-1,y)); }
        if y != 0 { vec.push((x,y-1)); }
        if x != self.width-1 { vec.push((x+1,y)); }
        if y != self.height-1 { vec.push((x,y+1)); }
        return vec
    }
}

struct Tile {
    height: u8, known: bool, valley: bool, basin: bool
}

impl Tile {
    fn new(height: u8) -> Tile {
        let known = height == 9;
        return Tile { height, known, valley: false, basin: false }
    }
}