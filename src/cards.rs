pub mod cards {

    #[derive(Debug)]
    pub struct Card {
        name: String,
        height: Vec<i8>,
        ticker: String
    }

    impl Card {
        pub fn new(name: String, height: Vec<i8>, ticker: String) -> Self {
            Card { name, height, ticker }
        }

        pub fn get_height(&self) -> &Vec<i8> {
            &self.height
        }

        pub fn get_ticker(&self) -> &String {
            &self.ticker
        }

        pub fn show_card(&self) {

            if self.name == "Ten" {
                println!("- - - -");
                println!("|     |");
                println!("| {}  |", self.ticker);
                println!("|     |");
                println!("- - - -");
            } else {
                println!("- - - -");
                println!("|     |");
                println!("|  {}  |", self.ticker);
                println!("|     |");
                println!("- - - -");
            }
        }
    }

}

#[cfg(test)]
mod tests {
    use super::cards::Card;

    
    #[test]
    fn test_get_height() {
        let card = Card::new("As".to_string(), vec![1, 11], "A".to_string());
        assert_eq!(card.get_height(), &[1, 11]);
    }
    
    #[test]
    fn test_get_ticker() {
        let card = Card::new("As".to_string(), vec![1, 11], "A".to_string());
        assert_eq!(card.get_ticker(), &"A".to_string());
    }

}