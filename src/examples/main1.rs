// how to include other files
mod side1;
#[path = "puzzles/a20_20.rs"] mod a20_20;

// allows us to leave out std:: when using i8::
use std::i8;

// default function, always there
fn main() {

    let current = a20_20::run;
    let file: &str = "a20_20.txt";
    let mut url: String = "C:/Users/rsoms/OneDrive/Documents/Projects/Rust/practice/src/puzzles/input/".to_owned();
    url.push_str(file);

    // immutable variable (default)
    let variable1 = 1;
    println!("Hello, world! {}", variable1);

    // mutable variable
    let mut variable = 1;
    println!("{}", variable);
    variable += 2;
    println!("{}", variable);

    // if statements (also shorthand)
    let age: u8 = 19;
    let older_than_18 = if age > 18 {true} else {false};
    if older_than_18 {
        println!("Older than 18!");
     }

    // tuple, list with up to 12 entries 
    let tuple1: (&str, bool, u8) = ("hey", true, std::u8::MAX);
    println!("tuples! {} {} {}", tuple1.0, tuple1.1, tuple1.2);

    // array, fixed length list, all same data type
    let array1: [i8; 3] = [i8::MIN, 16, i8::MAX];
    println!("array! {} {} {}", array1[0], array1[1], array1[2]);
    println!("easier array print! {:?}", array1);

    // slice of array, e.g. first 2 numbers of array1 (0 and 1, NOT 2)
    let slice1: &[i8] = &array1[0..2];
    println!("easier slice print! {:?}", slice1);

    // vectors, variable length list, all same data type
    let mut vector1: Vec<u8> = vec![std::u8::MIN, 16, 32, 64, std::u8::MAX];
    println!("easier vector print! {:?}", vector1);

    // popping and pushing from vectors
    let tmp_val: u8 = vector1[vector1.len()-1];
    vector1.pop();
    vector1.push(128);
    vector1.push(tmp_val);
    println!("added number to vector! {:?}", vector1);

    // loop through vector
    for x in vector1.iter() {
        println!("Print all vars in loop: {}", x);
    }

     // loop through vector and add
     vector1.pop();
     for x in vector1.iter_mut() {
        *x += 1;
    }
    println!("vector +1! {:?}", vector1);

    //loops in general
    loop {
        //keeps going until
        break;
    }

    // loops until i = 2
    for i in 0..3 {
        println!("for count loop! {}", i);
    }

    let mut count: u8 = 1;

    while count <4 {
        println!("while count loop! {}", count);
        count += 1;
    }

    for x in 1..4 {
        println!("for count loop! {}", x);
    }
    side1::run();

    // Pointers

    // Primitives (list/array gets copied)
    let mut array1 = [1, 2, 3];
    let mut array2 = array1;
    array1[2] = 5;
    array2[1] = 4;
    println!("{:?}", (array1, array2));

    // Non-primitives (references)
    let mut vector1: Vec<u8> = vec![1, 2, 3];
    // let mut vector2 = vector1; <-- won't work, vector1 becomes unassigned
    let vector2 = &mut vector1;
    //vector1[2] = 5; //<-- can't do both (no idea)
    vector2[1] = 4; //<-- can't do both (no idea)
    println!("Hey {:?}", (vector2));


    // 1 mut
    // many non-mut
    // owner
    // * = look at value at 5000
    // & = return 5000

    //stack/heap allocation (array = stack allocated, heap)

    // Get arguments from command line (i.e. "cargo run hello" prints hello)
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        let first_argument = args[1].clone();
        println!("passed on in cli: {}", first_argument);
     }
    
     current(&url);
}
