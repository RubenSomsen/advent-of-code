use std::collections::HashMap;

pub fn run(file_location: &str) {
    println!("NOW RUNNING: {}", file!());

    let vector: Vec<HashMap<String, String>> = parse_file(file_location);

    use std::time::Instant;
    let now = Instant::now();

    let answer_a: u16 = answer(&vector, true);
    let answer_b: u16 = answer(&vector, false);

    let elapsed = now.elapsed();
    println!("PERFORMANCE: {:?}", elapsed);

    println!("ANSWER A: {}", answer_a);
    println!("ANSWER B: {}", answer_b);
}

fn answer(vector: &Vec<HashMap<String, String>>, is_answer_a: bool) -> u16 {
    let mut answer: u16 = 0;
    for hashmap in vector {
        match hashmap.len() {
            8 => if is_answer_a || validity_check(&hashmap) { answer += 1 },
            7 => if hashmap.get("cid") == None && (is_answer_a || validity_check(&hashmap)) { answer += 1 },
            _ => continue
        }
    }
    return answer
}

fn validity_check(hashmap: &HashMap<String, String>) -> bool {
    let a1: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    let a2: [char; 16] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f'];
    let a3: [&str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
    for k in hashmap.keys() {
        let v: String = hashmap.get(k).unwrap().to_owned();
        if a1[0..3].contains(&&k[..]) {
            let e1: u16 = v.parse().unwrap();
            if k == a1[0] && (e1 < 1920 || e1 > 2002) { return false }
            else if k == a1[1] && (e1 < 2010 || e1 > 2020) { return false }
            else if k == a1[2] && (e1 < 2020 || e1 > 2030) { return false }
        }
        else if k == a1[3] { // ok
            let cm_in: String = v[v.len()-2..].to_owned();
            if cm_in != "cm" && cm_in != "in" { return false }
            let rest: u16 = v[..v.len()-2].parse::<u16>().unwrap();
            if cm_in == "cm" && (rest < 150 || rest > 193) { return false }
            if cm_in == "in" && (rest < 59 || rest > 76) { return false } 
        }
        else if k == a1[4] { // ok
            if v.len() != 7 { return false }
            for (i, c) in v.chars().enumerate() {
                if i == 0 { 
                    if c != '#' { return false }
                }
                else {
                    if !a2.contains(&c) { return false }
                }
            }
        }
        else if k == a1[5] && !a3.contains(&&v[..]) { return false } // ok
        else if k == a1[6] { // !!
            if v.len() != 9 { return false }
            for c in v.chars() {
                let _array = &a2[0..10];
                let _bool = _array.contains(&c);
                if !_bool { return false }
            }
        }
    }
    true
}

fn parse_file(file_name: &str) -> Vec<HashMap<String, String>> {
    use std::fs;
    let file = fs::read_to_string(file_name).expect("Unable to read file");

    let mut vector: Vec<HashMap<String, String>> = vec![];
    let passports = file.split("\r\n\r\n");
    for passport in passports {
        let mut hashmap = HashMap::new();
        let entries = passport.split(" ");
        for entry in entries {
            let sub_entries = entry.split("\r\n");
            for sub_entry in sub_entries {
                let mut entry_parts = sub_entry.split(':');
                hashmap.insert(entry_parts.next().unwrap().to_owned(), entry_parts.next().unwrap().to_owned());
            }
        }
        vector.push(hashmap);
    }
    return vector
}