use rand::{ rng, seq::SliceRandom };

#[derive(Debug)]
struct Deck {
    cards: Vec<String>,
}

impl Deck {
    fn new() -> Self {
        let suits = ["Hearts", "Spades", "Diamonds"];
        let values = ["Ace", "Two", "Three"];
        let mut cards = vec![];

        //Double nested for loop
        for suit in suits {
            for value in values {
                let card = format!("{value} of {suit}");
                cards.push(card);
            }
        }

        Deck { cards }
    }
    fn shuffle(&mut self) {
        let mut rng_value = rng();
        self.cards.shuffle(&mut rng_value);
    }
}

fn main() {
    let mut deck = Deck::new();
    deck.shuffle();
    println!("Heres your deck: {:#?}", deck);
}
