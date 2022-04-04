pub fn run(file_location: &str) {
    println!("NOW RUNNING: {}", file!());

    let mut scanners = parse_file(file_location);
    
    use std::time::Instant;
    let now = Instant::now();

    let a = answer_a(&mut scanners);
    let b = 0; //answer_b(&mut vec);

    let elapsed = now.elapsed();
    println!("PERFORMANCE: {:?}", elapsed);

    println!("ANSWER A: {:?}", a);
    println!("ANSWER B: {:?}", b);
}

fn answer_a(scanners: &mut Vec<Scanner>) -> usize {
    let ss1 = &scanners[0];
    let ss2 = &scanners[1];
    for s1 in &ss1.beacons {
        for s2 in &ss2.beacons {
            let mut counter = 0;
            let mut s1list = vec![];
            let mut s2list = vec![];
            for (i1, s1r) in ss1.get_relative_distance(&s1).iter().enumerate() {
                for (i2, s2r) in ss2.get_relative_distance(&s2).iter().enumerate() {
                    if s1r == s2r {
                        counter += 1;
                        s1list.push(&ss1.beacons[i1]);
                        s2list.push(&ss2.beacons[i2]);
                        break
                    }
                }
            }
            if counter > 11 { 
                let rel1 = s1list[0].relative_to(s1list[1]);
                for i in 0..24 {
                    let rel2 = s2list[0].transform(i).relative_to(&s2list[1].transform(i));
                     if rel1.equals(&rel2) {
                        let location = Some(s2list[0].transform(i).relative_to(&s1list[0]));
                        scanners[0].orientation = Orientation::new(i);
                        scanners[1].location = location;
                        println!("{:?} {:?} {:?}", scanners[1].location, 0, 0);
                        return 0
                    }
                }
            }
        }
    }
    return 1
}

fn parse_file(file_name: &str) -> Vec<Scanner> {
    use std::fs;
    let file = fs::read_to_string(file_name).unwrap();
    let mut scanner_vec = vec![];
    for scanner in file.split("\r\n\r\n") {
        let mut coord_vec = vec![];
        for (i, line) in scanner.lines().enumerate() {
            if i == 0 { continue }
            let xyz: Vec<i16> = line.split(",").map(|v| v.parse().unwrap()).collect();
            coord_vec.push(Coordinate::new(xyz[0], xyz[1], xyz[2]));
        }
        scanner_vec.push(Scanner::new(coord_vec));
    }
    scanner_vec[0].location = Some(Coordinate::new(0,0,0)); // treat as center
    return scanner_vec
}

#[derive(Debug)]
struct Coordinate {
    x: i16, y: i16, z: i16
}

impl Coordinate {
    fn new(x: i16, y: i16, z: i16) -> Coordinate {
        return Coordinate { x, y, z }
    }

    fn transform(&self, orientation: u8) -> Coordinate {
        let orientation = Orientation::new(orientation);
        let mut arr = [0,0,0];
        let pos = 
                  [[0,1,2],[1,3,2],[3,4,2],[4,0,2],  // z
                   [3,1,5],[1,0,5],[0,4,5],[4,3,5],  // -z
                   [0,5,1],[5,3,1],[3,2,1],[2,0,1],  // y
                   [3,5,4],[5,0,4],[0,2,4],[2,3,4],  // -y
                   [5,1,0],[1,2,0],[2,4,0],[4,5,0],  // x
                   [2,1,3],[1,5,3],[5,4,3],[4,2,3]]; // -x
        let pos = pos[orientation.get()];
        for i in 0..3 {
            arr[i] = match pos[i] {
                0 =>  self.x,
                1 =>  self.y,
                2 =>  self.z,
                3 => -self.x,
                4 => -self.y,
                _ => -self.z
            }
        }
        return Coordinate::new(arr[0], arr[1], arr[2])
    }

    fn relative_to(&self, c: &Coordinate) -> Coordinate {
        return Coordinate::new(c.x-self.x,c.y-self.y,c.z-self.z)
    }

    fn equals(&self, c: &Coordinate) -> bool {
        return c.x==self.x && c.y==self.y && c.z==self.z
    }

    fn distance(&self) -> usize {
        let (x, y, z) = (self.x.abs() as usize, self.y.abs() as usize, self.z.abs() as usize);
        return x*x+y*y+z*z
    }

    fn print24(&self) {
        for i in 0..23 {
            let c = self.transform(i);
            println!("{:?},{:?},{:?}", c.x, c.y, c.z);
        }
        println!();
    }
}

struct Orientation(u8);

impl Orientation {
    fn new(_0_23: u8) -> Orientation {
        if _0_23>23 { panic!("Exceeds number of possible orientations") }
        return Orientation(_0_23)
    }

    fn get(&self) -> usize {
        return self.0 as usize
    }
}

struct Scanner {
    beacons: Vec<Coordinate>, location: Option<Coordinate>, orientation: Orientation
}

impl Scanner {
    fn new(beacons: Vec<Coordinate>) -> Scanner {
        return Scanner { beacons, location: None, orientation: Orientation::new(0) }
    }

    fn get_relative_distance(&self, c: &Coordinate) -> Vec<usize> {
        let mut relative_coords = vec![];
        for beacon in &self.beacons {
            relative_coords.push(c.relative_to(&beacon).distance());
        }
        return relative_coords
    }
}