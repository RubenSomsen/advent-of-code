pub fn run(file_location: &str) {
    println!("NOW RUNNING: {}", file!());

    let mut vector: Vec<Vec<Location>> = parse_file(file_location);

    use std::time::Instant;
    let now = Instant::now();

    let answer_a: u32 = answer(&mut vector, false);
    let answer_b: u32 = answer(&mut vector, true);

    let elapsed = now.elapsed();
    println!("PERFORMANCE: {:?}", elapsed);

    println!("ANSWER A: {}", answer_a);
    println!("ANSWER B: {}", answer_b);
}

fn answer(row_vector: &mut Vec<Vec<Location>>, skip_empty: bool) -> u32 {
    let mut running = true;
    let mut boolean = true;
    reset_seats(row_vector);
    while running {
        if boolean { running = occupy_seats(row_vector, skip_empty) }
        else { running = leave_seats(row_vector, skip_empty) }
        boolean = !boolean;
    }
    occupy_seats(row_vector, skip_empty);
    return count_occupied(row_vector);
}

fn occupy_seats(row_vector: &mut Vec<Vec<Location>>, skip_empty: bool) -> bool {
    for y in 0..row_vector.len() {
        for x in 0..row_vector[0].len() {
            let vxy: &Location = &row_vector[y][x];
            if vxy.is_seat && !vxy.is_occupied && vxy.occupied_adjacent(&row_vector, skip_empty) == 0 { row_vector[y][x].occupy(); }
        }
    }
    return reset_updates(row_vector)
}

fn leave_seats(row_vector: &mut Vec<Vec<Location>>, skip_empty: bool) -> bool {
    for y in 0..row_vector.len() {
        for x in 0..row_vector[0].len() {
            let vxy: &Location = &row_vector[y][x];
            if vxy.is_seat && vxy.is_occupied && vxy.occupied_adjacent(&row_vector, skip_empty) >= 4 + skip_empty as u8 { row_vector[y][x].leave(); }
        }
    }
    return reset_updates(row_vector)
}

fn reset_updates(row_vector: &mut Vec<Vec<Location>>) -> bool {
    let mut is_success: bool = false;
    for y in 0..row_vector.len() {
        for x in 0..row_vector[0].len() {
            if row_vector[y][x].reset() { is_success = true; }
        }
    }
    return is_success
}

fn reset_seats(row_vector: &mut Vec<Vec<Location>>) {
    for y in 0..row_vector.len() {
        for x in 0..row_vector[0].len() {
            if row_vector[y][x].leave() {row_vector[y][x].reset(); }
        }
    }
}

fn count_occupied(row_vector: &Vec<Vec<Location>>) -> u32 {
    let mut count: u32 = 0;
    for row in row_vector {
        for location in row {
            if location.is_occupied { count += 1; }
        }
    }
    return count
}

fn is_in_bounds(x: i8, y: i8, v: &Vec<Vec<Location>>) -> bool {
    return !(x<0 || y<0 || x>=v[0].len() as i8 || y>=v.len() as i8)
}

fn draw(row_vector: &Vec<Vec<Location>>) {
    for y in 0..row_vector.len() {
        let mut string: String = "".to_owned();
        for x in 0..row_vector[0].len() {
            let loc = &row_vector[y][x];
            if loc.is_seat { 
                if loc.is_occupied {
                    string.push('#');
                }
                else {
                    string.push('L');
                }
            }
            else {
                string.push('.');
            }
        }
        println!("{}", string);
    }
    println!("");
}

fn parse_file(file_name: &str) -> Vec<Vec<Location>> {
    use std::fs;
    let file = fs::read_to_string(file_name).expect("Unable to read file");

    let mut row_vector: Vec<Vec<Location>> = vec![];
    for (y, row) in file.lines().enumerate() {
        let mut location_vector: Vec<Location> = vec![];
        for (x, location) in row.chars().enumerate() {
            let is_seat = location == 'L';
            let loc = Location::new(is_seat, x as i8, y as i8);
            location_vector.push(loc);
        }
        row_vector.push(location_vector);
    }
    return row_vector
}

struct Location {
    is_seat: bool, is_occupied: bool, is_updated: bool, x: i8, y: i8
}

impl Location {
    fn new(is_seat: bool, x: i8, y: i8) -> Location {
        return Location { is_seat: is_seat, is_occupied: false, is_updated: false, x: x, y: y }
    }

    fn occupy(&mut self) -> bool {
        if !self.is_seat || self.is_occupied { return false }
        self.is_occupied = true;
        self.is_updated = true;
        return true
    }

    fn leave(&mut self) -> bool {
        if !self.is_seat || !self.is_occupied { return false }
        self.is_occupied = false;
        self.is_updated = true;
        return true

    }
    fn reset(&mut self) -> bool {
        if !self.is_updated { return false }
        self.is_updated = false;
        return true 
    }

    fn check_empty(&self) -> bool {
        return !self.is_seat || (self.is_occupied == self.is_updated)
    }

    fn occupied_adjacent(&self, v: &Vec<Vec<Location>>, skip_empty: bool) -> u8 {
        let mut xy_array: [[i8; 2]; 8] = [[-1,-1],[ 0,-1],[ 1,-1],
                                          [-1, 0]        ,[ 1, 0],
                                          [-1, 1],[ 0, 1],[ 1, 1]];
        
        for xy in &mut xy_array {
            let (x, y): (i8, i8) = (xy[0], xy[1]);
            let (xx, yy): (i8, i8) = (self.x, self.y);
            let mut i: i8 = 1;
            while skip_empty && is_in_bounds(xx + x*i, yy + y*i, v) && !v[(yy + y*i) as usize][(xx + x*i) as usize].is_seat { i += 1; }
            xy[0] = xx + x*i;
            xy[1] = yy + y*i;
        }

        let mut count = xy_array.len() as u8;
        for xy in xy_array.iter() {
            let (x, y): (i8, i8) = (xy[0], xy[1]);
            if !is_in_bounds(x, y, v) || y>=v.len() as i8 || v[y as usize][x as usize].check_empty() { count -= 1 }
        }
        return count
    }
}