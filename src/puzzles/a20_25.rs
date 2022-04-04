const MOD: u32 = 20201227;
const SUBJ: u32 = 7; // generator
const PUBKEY_A: u32 = 10604480;//5764801; // card
const PUBKEY_B: u32 = 4126658;//17807724; // door

pub fn run(_file_location: &str) {
    println!("NOW RUNNING: {}", file!());

    let key_a = Key::new(PUBKEY_A);
    let key_b = Key::new(PUBKEY_B);

    use std::time::Instant;
    let now = Instant::now();

    let a = answer_a(key_a, key_b);
    //let b = answer_b(&mut map_b);

    let elapsed = now.elapsed();
    println!("PERFORMANCE: {:?}", elapsed);

    println!("ANSWER A: {:?}", a);
    //println!("ANSWER B: {:?}", b);
}

fn answer_a(mut key_a: Key, key_b: Key) -> u32 {
    //key_a.privkey = key_a.get_privkey();
    let dhkey = key_a.get_dhkey(key_b.pubkey);
    return dhkey
}

#[derive(Debug, Eq, PartialEq)]
struct Key {
    pubkey: u32, privkey: u32, subject: u32
}

impl Key {
    fn new(pubkey: u32) -> Key {
        let key = Key { pubkey: pubkey, privkey: 0, subject: SUBJ };
        key.set_privkey();
        return key
    }

    fn set_privkey(&self) -> u32 {
        let mut loop_count: u32 = 0;
        let mut pubkey = 1;
        loop {
            loop_count += 1;
            pubkey = (pubkey*self.subject)%MOD;
            if pubkey == self.pubkey { break }
        }
        return loop_count
    }

    //diffie-hellman
    fn get_dhkey(&self, pubkey: u32) -> u32 {
        let mut dhkey = 1;
        for _ in 0..self.privkey {
            dhkey = ((dhkey as u64*pubkey as u64)%MOD as u64) as u32
        }
        return dhkey
    }
}