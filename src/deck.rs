pub mod deck {
    use crate::cards::cards::Card;

    pub struct Deck {}

    impl Deck {
        pub fn new() -> Self {
            Deck {  }
        }

        pub fn create_deck(&self) -> Vec<Card> {
            let mut new_deck: Vec<Card> = Vec::new();
            let name: [&str; 13] = [
                "Aces", "Two", "Three", "Four", "Five", "Six",
                "Seven", "Eight", "Nine", "Ten", "Valet", 
                "Queen", "King"
            ];
            let value: [Vec<u8>; 13] = [
                vec![1, 11], vec![2], vec![3], vec![4], vec![5], vec![6],
                vec![7], vec![8], vec![9], vec![10], vec![10], vec![10],
                vec![10],
            ];
            let tickers: [&str; 13] = ["A", "2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K"];

            for i in 1..13 {
                new_deck.push(Card::new(name[i].to_string(), value[i].clone(), tickers[i].to_string()));
                new_deck.push(Card::new(name[i].to_string(), value[i].clone(), tickers[i].to_string()));
                new_deck.push(Card::new(name[i].to_string(), value[i].clone(), tickers[i].to_string()));
                new_deck.push(Card::new(name[i].to_string(), value[i].clone(), tickers[i].to_string()));
            }

            new_deck
        }
    }
}