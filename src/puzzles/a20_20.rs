pub fn run(file_location: &str) {
    println!("NOW RUNNING: {}", file!());

    let mut tiles = parse_file(file_location);

    let monster: [(usize,usize); 15] = [(18,0),(0,1),(5,1),(6,1),(11,1),(12,1),(17,1),(18,1),(19,1),(1,2),(4,2),(7,2),(10,2),(13,2),(16,2)];
    let monster_size: Coord = Coord { x:19, y:2 };
    
    use std::time::Instant;
    let now = Instant::now();

    let (corners, cube, offset) = form_cube(&mut tiles);
    let (total, tile) = cube_to_tile(cube, offset);
    let monsters = count_monsters(tile, &monster, &monster_size);

    let elapsed = now.elapsed();
    println!("PERFORMANCE: {:?}", elapsed);

    println!("ANSWER A: {:?}", corners);
    println!("ANSWER B: {:?}", total - monsters*monster.len() as u16);
}

fn count_monsters(mut tile: Tile, monster: &[(usize,usize); 15], monster_size: &Coord) -> u16 {
    for variant in 0..8 {
        tile.set_variant(variant);
        let mut count = 0;
        for y in 0..tile.get_size()-monster_size.y {
            'x: for x in 0..tile.get_size()-monster_size.x {
                for xy in monster {
                    let (xx, yy) = (xy.0 + x, xy.1 + y);
                    if !tile.is_occupied(&mut Coord { x:xx, y:yy }) { continue 'x }
                }
                count += 1;
            }
        }
        if count>1 { return count }
    }
    return 0
}

fn cube_to_tile(cube: Vec<Vec<Tile>>, offset: Coord) -> (u16, Tile) {
    let mut total = 0;
    let c_len = cube.len();
    let t_len = cube[0][0].vector.len()-2;
    let len = c_len*t_len;
    let mut vector: Vec<Vec<bool>> = vec![vec![false; len]; len];
    for yc in 0..c_len {
        for xc in 0..c_len {
            let tile = &cube[(yc+offset.y)%c_len][(xc+offset.x)%c_len];
            for yt in 0..t_len {
                for xt in 0..t_len {                             // +1,+1 to take off edges
                    let is_occupied = tile.is_occupied(&mut Coord { x:xt+1, y:yt+1 });
                    vector[yc*t_len+yt][xc*t_len+xt] = is_occupied;
                    if is_occupied { total += 1; }
                }                                                            
            }
        }
    }
    return (total, Tile::new(9999,0,vector))
}

fn form_cube(tiles: &mut Vec<Tile>) -> (u64, Vec<Vec<Tile>>, Coord) {    
    let mut answer: u64 = 1;
    let len = (tiles.len() as f32).sqrt() as usize;
    let mut cube_vector: Vec<Vec<Tile>> = vec![vec![Tile::new(0,0,vec![]); len]; len];
    cube_vector[0][0] = tiles[0].clone();
    let mut search_vector = vec![Coord {x:0, y:0}];
    let mut offset = Coord { x:0, y:0 };
    while search_vector.len()>0 {
        let curr = search_vector.pop().unwrap();     
        for side in 0..4 {
            let mut targ = curr.clone();
            if side == 0 { targ.y = (len+targ.y-1)%len; } // move up
            else if side == 1 { targ.x = (targ.x+1)%len; } // right
            else if side == 2 { targ.y = (targ.y+1)%len; } // down
            else if side == 3 { targ.x = (len+targ.x-1)%len; } // left
            if cube_vector[targ.y][targ.x].uid != 0 { continue } // continue if already occupied
            let (neighbor, has_neighbor) = cube_vector[curr.y][curr.x].find_neighbor(side, tiles);
            if !has_neighbor { // calculate modulo offset for 0,0
                if side == 0 { offset.y = curr.y; } // this line might be superfluous
                else if side == 1 { offset.x = (curr.x+1)%len; } // as well as this one
                else if side == 2 { offset.y = (curr.y+1)%len; }
                else if side == 3 { offset.x = curr.x; }
                continue
            }
            cube_vector[targ.y][targ.x] = neighbor;
            search_vector.push(targ);
        }
    }
    answer *= cube_vector[offset.y%len][offset.x%len].uid as u64; // up left
    answer *= cube_vector[offset.y%len][(len-1+offset.x)%len].uid as u64; //up right
    answer *= cube_vector[(len-1+offset.y)%len][(len-1+offset.x)%len].uid as u64; // down right
    answer *= cube_vector[(len-1+offset.y)%len][offset.x%len].uid as u64; // down left
    return (answer, cube_vector, offset)
}

fn _print_tile(tile: &Tile) { // just for debug
    println!("UID {:?}-{:?}", tile.uid, tile.variant);
    for y in 0..tile.vector.len() {
        for x in 0..tile.vector.len() {
            if tile.is_occupied(&mut Coord { x:x, y:y }) {  print!("#") }
            else {  print!(".") }
        }
        println!("");
    }
    println!("");
}

fn parse_file(file_name: &str) -> Vec<Tile> {
    use std::fs;
    let file = fs::read_to_string(file_name).expect("Unable to read file");

    let mut tile_vector: Vec<Tile> = vec![];
    for tile in file.split("\r\n\r\n") {
        let mut y_vec = vec![vec![false; 10]; 10];
        let mut uid: u16 = 0;
        for (y, line) in tile.lines().enumerate() {
            if y == 0 {
                uid = line[5..9].parse().unwrap();
                continue
            }
            for (x, c) in line.chars().enumerate() {
                if c == '#' {
                    y_vec[y-1][x] = true;
                }
            }
        }
        tile_vector.push(Tile::new(uid, 0, y_vec));
    }
    return tile_vector
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Tile {
    uid: u16, variant: u8, vector: Vec<Vec<bool>>
}

// "Variant" is a number from 0 to 7 and flips/turns the tile (transforming coordinates)
impl Tile {
    fn new(uid: u16, variant: u8, vector: Vec<Vec<bool>>) -> Tile {
        return Tile { uid, variant, vector }
    }

    fn set_variant(&mut self, variant: u8) {
        self.variant = variant
    }

    fn get_size(&self) -> usize {
        return self.vector.len()
    }

    // "Side" is the N, E, S or W side of the tile, depicted by 0, 1, 2 or 3 
    fn get_side(&self, side: u8) -> Vec<bool> {
        let mut side_vector = vec![];
        let len = self.vector.len();
        for i in 0..len {
            let mut c = Coord { x:0, y:0 };
            if side == 0 { c.x = i; }
            else if side == 1 { c.x = len-1; c.y = i; }
            else if side == 2 { c.x = i; c.y = len-1; }
            else if side == 3 { c.y = i; }
            side_vector.push(self.is_occupied(&mut c));
        }
        return side_vector
    }

    fn is_neighbor(&self, side: u8, neighbor_tile: &Tile) -> bool {
        let this_side = self.get_side(side);
        let neighbor_side = neighbor_tile.get_side((side+2)%4);
        for i in 0..this_side.len() {
            if !(this_side[i] == neighbor_side[i]) { return false }
        }
        return true
    }

    fn find_neighbor(&self, side: u8, tiles: &mut Vec<Tile>) -> (Tile, bool) {
        for t in 0..tiles.len() {
            let tile = &mut tiles[t];
            if tile.uid == self.uid { continue }
            for variant in 0..8 {
                tile.set_variant(variant);
                if self.is_neighbor(side, tile) {
                    return (tile.clone(), true)
                }
            }
        }
        return (Tile::new(0,0,vec![]), false)
    }

    // Transforms the coordinates depending on the variant
    fn is_occupied(&self, c: &mut Coord) -> bool {
        let variants = [self.variant & 1 != 0, self.variant & 2 != 0, self.variant & 4 != 0];
        let len = self.vector.len() as usize;
        if variants[0] { c.x = len-1-c.x }
        if variants[1] { c.y = len-1-c.y }
        if variants[2] { let tx = c.x; c.x = c.y; c.y = tx; }
        return self.vector[c.y][c.x]
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Coord {
    x: usize, y: usize
}