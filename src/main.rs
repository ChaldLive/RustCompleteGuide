use rand::{thread_rng, seq::SliceRandom};

#[derive(Debug)]
struct Deck{
    cards: Vec<String>,
}

impl Deck {
    fn new() -> Self {
        let suits = ["Hearts", "Diamonds", "Clubs", "Spades"];
        let values = ["Ace", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten", "Jack", "Queen", "King"];
        let mut cards = vec![];
        for suit in &suits {
            for value in &values {
                let card = format!("{} Of {}", value, suit);
                cards.push(card);
            }
        }
        Deck{cards}
    }
    fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }
    /* Missing error handling here. Next project in the course.*/
    fn deal(&mut self, num_cards: usize)  -> Vec<String>{
        self.cards.split_off(self.cards.len() - num_cards)
    }
}

fn main() {
    let mut deck = Deck::new();
    deck.shuffle();
    let player_one = deck.deal(13);
    println!("Here is your hand: {:#?}", player_one);
}
