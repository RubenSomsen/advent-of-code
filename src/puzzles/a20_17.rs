use std::collections::HashMap;

pub fn run(file_location: &str) {
    println!("NOW RUNNING: {}", file!());

    let mut map: HashMap<i16, HashMap<i16, HashMap<i16, Cube>>> = parse_file(file_location);
    let mut map_b: HashMap<i16, HashMap<i16, HashMap<i16, HashMap<i16, Bcube>>>> = parse_file_b(file_location);

    use std::time::Instant;
    let now = Instant::now();

    let a: u16 = answer_a(&mut map);
    let b: u16 = answer_b(&mut map_b);

    let elapsed = now.elapsed();
    println!("PERFORMANCE: {:?}", elapsed);

    println!("ANSWER A: {}", a);
    println!("ANSWER B: {}", b);
}

fn answer_a(zmap: &mut HashMap<i16, HashMap<i16, HashMap<i16, Cube>>>) -> u16 {
    let mut count = 0;
    for _ in 0..6 {
        count = update_state(zmap);
    }
    return count
}

fn update_state(zmap: &mut HashMap<i16, HashMap<i16, HashMap<i16, Cube>>>) -> u16 {
    let mut count: u16 = 0;
    for i in 0..2 {
        let cubes = get_all_cubes(zmap);
        for ii in 0..cubes.len() {
            let (x, y, z) = (cubes[ii][0], cubes[ii][1], cubes[ii][2]);
            let cube = get_cube(x as i16, y as i16, z as i16, zmap);
            if i==0 {
                if cube.active {
                    for xyz in cube.get_neighbors().iter() { get_cube(xyz[0], xyz[1], xyz[2], zmap).touch(); }
                }
            }
            else {
                if cube.neighbors == 3 || (cube.active && cube.neighbors == 2) { 
                    cube.active = true; 
                    count += 1;
                }
                else { cube.active = false; }
                cube.reset();
            }
        }
    }
    
    return count
}

fn get_all_cubes(zmap: &HashMap<i16, HashMap<i16, HashMap<i16, Cube>>>) -> Vec<[i16; 3]> {
    let mut vector: Vec<[i16; 3]> = vec![];
    for (z, ymap) in zmap {
        for (y, xmap) in ymap {
            for (x, cube) in xmap {
                vector.push([*x, *y, *z]);
            }
        }
    }
    return vector
}

fn get_cube(x: i16, y: i16, z: i16, zmap: &mut HashMap<i16, HashMap<i16, HashMap<i16, Cube>>>) -> &mut Cube {
    if zmap.get(&z) == None { zmap.insert(z, HashMap::new()); }
    let ymap = zmap.get_mut(&z).unwrap();
    if ymap.get(&y) == None { ymap.insert(y, HashMap::new()); }
    let xmap = ymap.get_mut(&y).unwrap();
    if xmap.get(&x) == None { xmap.insert(x, Cube::new(x, y, z)); }
    let cube = xmap.get_mut(&x).unwrap();
    return cube
}

fn parse_file(file_name: &str) -> HashMap<i16, HashMap<i16, HashMap<i16, Cube>>> {
    use std::fs;
    let file = fs::read_to_string(file_name).expect("Unable to read file");

    let mut zmap: HashMap<i16, HashMap<i16, HashMap<i16, Cube>>> = HashMap::new();

    let mut ymap: HashMap<i16, HashMap<i16, Cube>> = HashMap::new();
    for (y, line) in file.lines().enumerate() {
        let mut xmap: HashMap<i16, Cube> = HashMap::new();
        for (x, c) in line.chars().enumerate() {
            let mut cube = Cube::new(x as i16, y as i16, 0);
            if c == '#' { cube.active = true }
            xmap.insert(x as i16, cube);
        }
        ymap.insert(y as i16, xmap);
    }
    zmap.insert(0, ymap);
    return zmap
}

#[derive(Debug, Eq, PartialEq)]
struct Cube {
    active: bool, neighbors: u8, x: i16, y: i16, z: i16
}

impl Cube {
    fn new(x: i16, y: i16, z: i16) -> Cube {
        return Cube { active: false, neighbors: 0, x: x, y: y, z: z }
    }

    fn touch(&mut self) {
        self.neighbors += 1;
    }

    fn reset(&mut self) {
        self.neighbors = 0;
    }

    fn get_neighbors(&self) -> [[i16; 3]; 3*3*3-1] {
        let mut array: [[i16; 3]; 3*3*3-1] = [[0; 3]; 3*3*3-1];
        let v: [i16; 3] = [0, 1, -1];
        for z in 0..3 {
            for y in 0..3 {
                for x in 0..3 {
                    let i = x + y*3 + z*3*3; // trinary
                    if i == 0 { continue }
                    array[i-1] = [self.x+v[x], self.y+v[y], self.z+v[z]];
                }
            }
        }
        return array
    }
}


// Everything copied for answer B, but with another dimension added

fn answer_b(wmap: &mut HashMap<i16, HashMap<i16, HashMap<i16, HashMap<i16, Bcube>>>>) -> u16 {
    let mut count = 0;
    for _ in 0..6 {
        count = update_state_b(wmap);
    }
    return count
}

fn update_state_b(wmap: &mut HashMap<i16, HashMap<i16, HashMap<i16, HashMap<i16, Bcube>>>>) -> u16 {
    let mut count: u16 = 0;
    for i in 0..2 {
        let cubes = get_all_cubes_b(wmap);
        for ii in 0..cubes.len() {
            let (x, y, z, w) = (cubes[ii][0], cubes[ii][1], cubes[ii][2], cubes[ii][3]);
            let cube = get_cube_b(x as i16, y as i16, z as i16, w as i16, wmap);
            if i==0 {
                if cube.active {
                    for xyz in cube.get_neighbors().iter() { get_cube_b(xyz[0], xyz[1], xyz[2], xyz[3], wmap).touch(); }
                }
            }
            else {
                if cube.neighbors == 3 || (cube.active && cube.neighbors == 2) { 
                    cube.active = true; 
                    count += 1;
                }
                else { cube.active = false; }
                cube.reset();
            }
        }
    }
    
    return count
}

fn get_all_cubes_b(wmap: &HashMap<i16, HashMap<i16, HashMap<i16, HashMap<i16, Bcube>>>>) -> Vec<[i16; 4]> {
    let mut vector: Vec<[i16; 4]> = vec![];
    for (w, zmap) in wmap {
        for (z, ymap) in zmap {
            for (y, xmap) in ymap {
                for (x, cube) in xmap {
                    vector.push([*x, *y, *z, *w]);
                }
            }
        }
    }
    return vector
}

fn get_cube_b(x: i16, y: i16, z: i16, w: i16, wmap: &mut HashMap<i16, HashMap<i16, HashMap<i16, HashMap<i16, Bcube>>>>) -> &mut Bcube {
    if wmap.get(&w) == None { wmap.insert(w, HashMap::new()); }
    let zmap = wmap.get_mut(&w).unwrap();
    if zmap.get(&z) == None { zmap.insert(z, HashMap::new()); }
    let ymap = zmap.get_mut(&z).unwrap();
    if ymap.get(&y) == None { ymap.insert(y, HashMap::new()); }
    let xmap = ymap.get_mut(&y).unwrap();
    if xmap.get(&x) == None { xmap.insert(x, Bcube::new(x, y, z, w)); }
    let cube = xmap.get_mut(&x).unwrap();
    return cube
}

fn parse_file_b(file_name: &str) -> HashMap<i16, HashMap<i16, HashMap<i16, HashMap<i16, Bcube>>>> {
    use std::fs;
    let file = fs::read_to_string(file_name).expect("Unable to read file");

    let mut wmap: HashMap<i16, HashMap<i16, HashMap<i16, HashMap<i16, Bcube>>>> = HashMap::new();
    let mut zmap: HashMap<i16, HashMap<i16, HashMap<i16, Bcube>>> = HashMap::new();
    let mut ymap: HashMap<i16, HashMap<i16, Bcube>> = HashMap::new();
    for (y, line) in file.lines().enumerate() {
        let mut xmap: HashMap<i16, Bcube> = HashMap::new();
        for (x, c) in line.chars().enumerate() {
            let mut cube = Bcube::new(x as i16, y as i16, 0, 0);
            if c == '#' { cube.active = true }
            xmap.insert(x as i16, cube);
        }
        ymap.insert(y as i16, xmap);
    }
    zmap.insert(0, ymap);
    wmap.insert(0, zmap);
    return wmap
}

#[derive(Debug, Eq, PartialEq)]
struct Bcube {
    active: bool, neighbors: u8, x: i16, y: i16, z: i16, w: i16
}

impl Bcube {
    fn new(x: i16, y: i16, z: i16, w: i16) -> Bcube {
        return Bcube { active: false, neighbors: 0, x: x, y: y, z: z, w: w }
    }

    fn touch(&mut self) {
        self.neighbors += 1;
    }

    fn reset(&mut self) {
        self.neighbors = 0;
    }

    fn get_neighbors(&self) -> [[i16; 4]; 3*3*3*3-1] {
        let mut array: [[i16; 4]; 3*3*3*3-1] = [[0; 4]; 3*3*3*3-1];
        let v: [i16; 3] = [0, 1, -1];
        for w in 0..3 {
            for z in 0..3 {
                for y in 0..3 {
                    for x in 0..3 {
                        let i = x + y*3 + z*3*3 + w*3*3*3; // trinary
                        if i == 0 { continue }
                        array[i-1] = [self.x+v[x], self.y+v[y], self.z+v[z], self.w+v[w]];
                    }
                }
            }
        }
        return array
    }
}