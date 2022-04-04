pub fn run(file_location: &str) {
    println!("NOW RUNNING: {}", file!());

    let test_array = (20, 30, -10, -5);
    let array = (248, 285, -85, -56);
    let coords = Coordinates::new(array);
    
    use std::time::Instant;
    let now = Instant::now();

    let a = coords.y_max*(coords.y_max+1)/2; // lol
    let b = answer(&coords);

    let elapsed = now.elapsed();
    println!("PERFORMANCE: {:?}", elapsed);

    println!("ANSWER A: {:?}", a);
    println!("ANSWER B: {:?}", b);
}

fn answer(c: &Coordinates) -> isize {
    let mut counter = 0;
    for y in c.y_min..c.y_max+1 {
        for x in c.x_min..c.x_max+1 {
            counter += shoot(x, y, c) as isize;
        }
    }
    return counter
}

fn shoot(x: isize, y: isize, c: &Coordinates) -> bool {
    let (mut i, mut xx, mut yy) = (0, 0, 0);
    return loop {
        if x > i { xx += x-i }
        yy += y-i;
        if xx >= c.x_left && xx <= c.x_right && yy >= c.y_bottom && yy <= c.y_top { break true }
        else if xx>c.x_right || yy<c.y_bottom { break false }
        i += 1;
    }
}

struct Coordinates {
    x_left: isize, x_right: isize, y_bottom: isize, y_top: isize, y_min: isize, y_max: isize, x_min: isize, x_max: isize
}

impl Coordinates {
    fn new((x_left, x_right, y_bottom, y_top): (isize, isize, isize, isize)) -> Coordinates {
        let x_max = x_right;
        let y_min = y_bottom;
        let y_max = y_bottom.abs()-1;
        let mut x_min = 0;
        loop { x_min += 1; if x_min*(x_min+1)/2>x_left { break } };
        return Coordinates { x_left, x_right, y_bottom, y_top, y_min, y_max, x_min, x_max }
    }
}