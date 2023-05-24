mod cards;
mod deck;

use crate::cards::cards::Card;

fn main() {
    let card: Card = Card::new("Aces".to_string(), vec![1], "A".to_string());
    card.show_card()
}
