const DECK: u8 = 50; // 10 or 50, depending on input

pub fn run(file_location: &str) {
    println!("NOW RUNNING: {}", file!());

    let (mut deck1a, mut deck2a) = parse_file(file_location);
    let (mut deck1b, mut deck2b) = (deck1a.clone(), deck2a.clone());
    
    use std::time::Instant;
    let now = Instant::now();

    // WARNING: Won't work unless the const at the top is correctly set
    let a = play_game(&mut deck1a, &mut deck2a, false);
    let b = play_game(&mut deck1b, &mut deck2b, true);

    let elapsed = now.elapsed();
    println!("PERFORMANCE: {:?}", elapsed);

    println!("ANSWER A: {:?}", a);
    println!("ANSWER B: {:?}", b);
}

fn play_game(deck1: &mut Deck, deck2: &mut Deck, is_b: bool) -> (bool, u32) {
    let mut rounds: Vec<(u32, u32)> = vec![];
    loop {
        let deck1_score = deck1.get_score();
        let deck2_score = deck2.get_score();
        for round in &rounds { // Skipped for A
            if round.0 == deck1_score && round.1 == deck2_score {
                return (true, 0)
            }
        }
        if is_b { rounds.push((deck1_score, deck2_score)); } // Skipped for A
        let card1 = deck1.draw();
        let card2 = deck2.draw();
        let do_subgame = is_b && card1 <= deck1.pile && card2 <= deck2.pile;
        let mut subgame_p1_wins = false;
        if do_subgame { // Skipped for A
            subgame_p1_wins = play_game(&mut deck1.get_clone(card1), &mut deck2.get_clone(card2), is_b).0;
        }
        if (do_subgame && subgame_p1_wins) || (!do_subgame && card1>card2) {
            deck1.insert(card1);
            deck1.insert(card2);
        }
        else {
            deck2.insert(card2);
            deck2.insert(card1);
        }
        if deck1.has_lost() {
            return (false, deck2.get_score())
        }
        else if deck2.has_lost() {
            return (true, deck1.get_score())
        }
    }
}

fn parse_file(file_name: &str) -> (Deck, Deck) {
    use std::fs;
    let file = fs::read_to_string(file_name).expect("Unable to read file");

    let mut deck1 = Deck::new();
    let mut deck2 = Deck::new();
    let mut file_split = file.split("\r\n\r\n");
    let mut cards1 = file_split.next().unwrap().lines();
    let mut cards2 = file_split.next().unwrap().lines();
    cards1.next();
    cards2.next();
    for _ in 0..DECK/2 {
        deck1.insert(cards1.next().unwrap().parse().unwrap());
        deck2.insert(cards2.next().unwrap().parse().unwrap());
    }
    return (deck1, deck2)
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
struct Deck {
    pos: u8, pile: u8, cards: [u8; DECK as usize]
}

impl Deck {
    fn new() -> Deck {
        return Deck { pos: 0, pile: 0, cards: [0; DECK as usize] }
    }

    fn insert(&mut self, card: u8) {
        self.cards[((self.pos+self.pile)%DECK) as usize] = card;
        self.pile += 1;
    }

    fn draw(&mut self) -> u8 {
        let card = self.cards[self.pos as usize];
        self.cards[self.pos as usize] = 0;
        self.pile -= 1;
        self.pos = (self.pos+1)%DECK;
        return card
    }

    fn has_lost(&self) -> bool {
        return self.pile == 0
    }

    fn get_score(&self) -> u32 { // also doubles as uid
        let mut score = 0;
        for i in 0..self.pile {
            score += self.cards[((self.pos+i)%DECK) as usize] as u32*(self.pile-i) as u32;
        }
        return score
    }

    fn get_clone(&self, cards: u8) -> Deck {
        let mut clone = Deck::new();
        for i in 0..cards {
            let card = self.cards[((self.pos+i)%DECK) as usize];
            clone.insert(card);
        }
        return clone
    }
}