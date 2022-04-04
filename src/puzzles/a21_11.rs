pub fn run(file_location: &str) {
    println!("NOW RUNNING: {}", file!());

    let mut grid = parse_file(file_location);
    grid.print();
    
    use std::time::Instant;
    let now = Instant::now();

    let a = answer_a(&mut grid, 100);
    let b = answer_b(&mut grid, 100);

    let elapsed = now.elapsed();
    println!("PERFORMANCE: {:?}", elapsed);

    println!("ANSWER A: {:?}", a);
    println!("ANSWER B: {:?}", b);
}

fn answer_a(grid: &mut Grid, cycles: usize) -> usize {
    let mut counter = 0;
    for _ in 0..cycles {
        counter += grid.update(); 
    }
    return counter
}

fn answer_b(grid: &mut Grid, start: usize) -> usize {
    let mut i = start;
    loop {
        i += 1;
        if grid.update() == grid.width*grid.height {
            return i
        } 
    }
}

fn parse_file(file_name: &str) -> Grid {
    use std::fs;
    let file = fs::read_to_string(file_name).expect("Unable to read file");
    let mut grid = Grid::new();
    for (y, line) in file.lines().enumerate() {
        if grid.width == 0 { grid.width = line.len() }
        grid.height += 1;
        for (x, c) in line.chars().enumerate() {
            grid.state.push(Squid::new(c as u8 - 48, x, y));
        }
    }
    return grid
}

struct Grid {
    state: Vec<Squid>, width: usize, height: usize
}

impl Grid {
    fn new() -> Grid {
        return Grid { state: vec![], width: 0, height: 0 }
    }

    fn get(&mut self, x: usize, y: usize) -> &mut Squid {
        self.bounds_check(x,y);
        return &mut self.state[x+(y*self.width)]
    }

    fn touch_neighbors(&mut self, x: usize, y: usize) -> usize {
        self.bounds_check(x,y);
        let mut counter = 1;
        let vec = [(0,1),(1,0),(0,-1),(-1,0),(-1,1),(1,-1),(1,1),(-1,-1)];
        for xy in vec {
            let (xn, yn) = (x as isize + xy.0, y as isize +xy.1);
            if xn >= 0 && yn >= 0 && xn<self.width as isize && yn<self.height as isize {
                if self.get(xn as usize,yn as usize).touch() { 
                    counter += self.touch_neighbors(xn as usize, yn as usize);
                 }
            }   
        }
        return counter
    }

    fn bounds_check(&self, x: usize, y: usize) {
        if x>=self.width || y>=self.height { panic!("xy out of bounds!") }
    }

    fn update(&mut self) -> usize {
        let mut counter = 0;
        for i in 0..self.state.len() {
            if self.state[i].touch() {
                counter += self.touch_neighbors(self.state[i].x, self.state[i].y);
            }
        }
        for i in 0..self.state.len() {
            self.state[i].dim();
        }
        return counter
    }

    fn print(&self) {
        for (i, squid) in self.state.iter().enumerate() {
            if i%self.width == 0 { println!() }
            print!("{}", squid.brightness);
        }
        println!();
    }
}

struct Squid {
    brightness: u8, x: usize, y: usize
}

impl Squid {
    fn new(brightness: u8, x: usize, y: usize) -> Squid {
        return Squid { brightness, x, y }
    }

    fn touch(&mut self) -> bool {
        if self.brightness == 10 { return false }
        self.brightness += 1;
        if self.brightness == 10 { return true }
        return false
    }

    fn dim(&mut self) {
        if self.brightness == 10 { self.brightness = 0 }
    }
}