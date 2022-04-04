// In "Cargo.toml", put:
// [dependencies]
// hex = "0.4.3"
use hex;

pub fn run(file_location: &str) {
    println!("NOW RUNNING: {}", file!());

    let mut bit_handler = parse_file(file_location);
    
    use std::time::Instant;
    let now = Instant::now();

    let a = answer_a(&mut bit_handler, &mut 0);
    let b = answer_b(&mut bit_handler, &mut 0);

    let elapsed = now.elapsed();
    println!("PERFORMANCE: {:?}", elapsed);

    println!("ANSWER A: {:?}", a);
    println!("ANSWER B: {:?}", b);
}

fn answer_a(bit_handler: &mut BitHandler, bit_index: &mut usize) -> u64 {
    let mut version_counter = 0;
    loop {
        version_counter += bit_handler.get_bits(bit_index, 3);
        if *bit_index >= bit_handler.byte_vec.len()*8 { return version_counter }
        let type_id = bit_handler.get_bits(bit_index, 3);
        if type_id == 4 {  bit_handler.get_var_int(bit_index); }
        else {
            let is_true_bit = bit_handler.get_bits(bit_index, 1) == 1;
            bit_handler.get_bits(bit_index, if is_true_bit { 11 } else { 15 });
        }
    }
}

fn answer_b(bit_handler: &mut BitHandler, bit_index: &mut usize) -> u64 {
    let _version = bit_handler.get_bits(bit_index, 3);
    let type_id = bit_handler.get_bits(bit_index, 3);
    if type_id == 4 {
        return bit_handler.get_var_int(bit_index);
    }
    else {
        let is_true_bit = bit_handler.get_bits(bit_index, 1) == 1;
        let mut limit = bit_handler.get_bits(bit_index, if is_true_bit { 11 } else { 15 });
        let mut answer_vec: Vec<u64> = vec![];
        if is_true_bit {
            for _ in 0..limit {
                answer_vec.push(answer_b(bit_handler, bit_index));
            }
        }
        else {
            while limit != 0 {
                let current = *bit_index as u64;
                answer_vec.push(answer_b(bit_handler, bit_index));
                limit -= (*bit_index as u64) - current;
            }
        }
        return if type_id == 0 {  answer_vec.iter().sum() }
          else if type_id == 1 {  answer_vec.iter().product() }
          else if type_id == 2 { *answer_vec.iter().min().unwrap() }
          else if type_id == 3 { *answer_vec.iter().max().unwrap() }
          else if type_id == 5 { (answer_vec[0] > answer_vec[1]) as u64 }
          else if type_id == 6 { (answer_vec[0] < answer_vec[1]) as u64 }
          else if type_id == 7 { (answer_vec[0] == answer_vec[1]) as u64 }
          else { panic!("Input data invalid. Should never be reached.") }
    }
}

fn parse_file(file_name: &str) -> BitHandler {
    use std::fs;
    let file = fs::read_to_string(file_name).expect("Unable to read file");
    let byte_vec = hex::decode(file).unwrap();
    let bit_handler = BitHandler::new(byte_vec);
    return bit_handler
}

struct BitHandler {
    byte_vec: Vec<u8>
}

impl BitHandler {
    fn new(byte_vec: Vec<u8>) -> BitHandler {
        return BitHandler { byte_vec }
    }

    fn get_bits(&mut self, bit_index: &mut usize, amount: usize) -> u64 {
        if amount>64 { panic!("Can't return more than 64 bits.") }
        if *bit_index + amount > self.byte_vec.len()*8 { return 0 }
        let mut bits = 0;
        for _ in 0..amount {
            let byte = self.byte_vec[(*bit_index)/8];
            let remainder = (*bit_index)%8;
            let is_true = byte & 1 << 7-remainder != 0;
            bits = bits << 1 | is_true as u64;
            *bit_index += 1;
        }
        return bits
    }

    fn get_var_int(&mut self, bit_index: &mut usize) -> u64 {
        let mut bits = 0;
        loop {
            let first_bit = self.get_bits(bit_index, 1);
            let four_bits= self.get_bits(bit_index, 4);
            bits |= four_bits;
            if first_bit == 0 { return bits }
            bits = bits << 4;
        }
    }
}