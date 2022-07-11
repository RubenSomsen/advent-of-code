// Contains puzzles from 2019, 2020 (complete), 2021
// Just change a00_00 to year and puzzle, e.g. a19_01

#[path = "./puzzles/a21_18.rs"] mod m2118;

fn main() {

    use std::time::Instant;
    let now = Instant::now();

    let path = std::env::current_dir().unwrap();
    let full_path = path.to_str().unwrap().to_owned() + "/src/puzzles/input/";
    let extension = ".txt";
    
    m2118::run(&(full_path.to_owned() + "a21_18" + extension));

    let elapsed = now.elapsed();
    println!("PERFORMANCE TOTAL: {:?}", elapsed);
}