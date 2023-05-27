pub mod deck {
    use crate::cards::cards::Card;
    use rand::seq::SliceRandom;

    pub struct Deck {
        deck: Vec<Card>
    }

    impl Deck {
        pub fn new() -> Self {
            Deck { deck: vec![] }
        }

        pub fn get_card(&mut self) -> Option<Card> {
            self.deck.pop()
        }

        pub fn create_deck(&mut self) -> bool {
            let mut new_deck: Vec<Card> = Vec::new();
            let name: [&str; 13] = [
                "Aces", "Two", "Three", "Four", "Five", "Six",
                "Seven", "Eight", "Nine", "Ten", "Valet", 
                "Queen", "King"
            ];
            let value: [Vec<i8>; 13] = [
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

            self.deck = new_deck;
            true
        }

        pub fn shuffle_deck(&mut self) -> bool {
            self.deck.shuffle(&mut rand::thread_rng());
            true
        }
    }
}

#[cfg(test)]
mod tests {
    use super::deck::Deck;

    #[test]
    fn test_get_card() {
        let mut deck = Deck::new();
        deck.create_deck();
        let card_1 = match  deck.get_card() {
            Some(val) => val,
            None => panic!()
        };

        assert_eq!(card_1.get_ticker(), &"K".to_string());

        let card_2 = match  deck.get_card() {
            Some(val) => val,
            None => panic!()
        };

        assert_eq!(card_2.get_ticker(), &"K".to_string());

        let card_3 = match  deck.get_card() {
            Some(val) => val,
            None => panic!()
        };

        assert_eq!(card_3.get_ticker(), &"K".to_string());

        let card_4 = match  deck.get_card() {
            Some(val) => val,
            None => panic!()
        };

        assert_eq!(card_4.get_ticker(), &"K".to_string());

        let card_5 = match  deck.get_card() {
            Some(val) => val,
            None => panic!()
        };

        assert_eq!(card_5.get_ticker(), &"Q".to_string());
    }

    #[test]
    fn test_create_deck() {
        let mut deck = Deck::new();
        let check = match  deck.get_card() {
            Some(_) => true,
            None => false
        };
        assert_eq!(check, false);

        deck.create_deck();
        let card_1 = match  deck.get_card() {
            Some(val) => val,
            None => panic!()
        };
        assert_eq!(card_1.get_ticker(), &"K".to_string());
    }
}