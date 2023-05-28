pub mod players {
    
    use crate::cards::cards::Card;

    pub struct Player {
        name: String,
        hand: Vec<Card>,
        balance: f32,
        wins: i32,
        defeats: i32
    }

    pub trait PlayerType {
        fn new(name: String, hand: Vec<Card>, balance: f32, wins: i32, defeats: i32) -> Self;
        fn get_name(&self) -> &String;
        fn get_hand(&self) -> &Vec<Card>;
        fn add_card_to_hand(&mut self, card: Card) -> bool;
        fn clear_hand(&mut self) -> bool;
        fn get_balance(&self) -> f32;
        fn deposit(&mut self, new_balance: f32) -> bool;
        fn withdraw(&mut self, value: f32) -> bool;
        fn get_wins(&self) -> i32;
        fn add_win(&mut self) -> bool;
        fn get_defeats(&self) -> i32;
        fn add_defeats(&mut self) -> bool;
        fn show_hand(&self);
    }

    impl PlayerType for Player {

        fn new(name: String, hand: Vec<Card>, balance: f32, wins: i32, defeats: i32) -> Self {
            Player { name, hand, balance, wins, defeats }
        }

        fn get_name(&self) -> &String { 
            &self.name 
        }

        fn get_hand(&self) -> &Vec<Card> {
            &self.hand
        }

        fn add_card_to_hand(&mut self, card: Card) -> bool {
            self.hand.push(card);
            true
        }

        fn clear_hand(&mut self) -> bool {
            self.hand = vec![];
            true
        }

        fn get_balance(&self) -> f32 {
            self.balance
        }

        fn deposit(&mut self, new_balance: f32) -> bool {
            self.balance += new_balance;
            true
        }

        fn withdraw(&mut self, value: f32) -> bool {
            self.balance -= value;
            true
        }

        fn get_wins(&self) -> i32 {
            self.wins
        }

        fn add_win(&mut self) -> bool {
            self.wins += 1;
            true
        }

        fn get_defeats(&self) -> i32 {
            self.defeats
        }

        fn add_defeats(&mut self) -> bool {
            self.defeats += 1;
            true
        }

        fn show_hand(&self) {
            for card in &self.hand {
                card.show_card();
            }
        }
    }

    impl Player {
        pub fn show_info(&self) {
            if self.balance > 0 as f32 {
                println!("{}, your balance is: {}", self.name, self.balance);
            } else if self.balance == 0 as f32 {
                println!("{}, you haven't balance, thanks for play BlackJack", self.name);
            } else {
                println!("{}, you're owe {}", self.name, self.balance)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use crate::{players::players::PlayerType, cards::cards::Card};

    use super::players::Player;

    #[test]
    fn test_get_name() {
        let player = Player::new("Test".to_string(), vec![], 0.0, 0, 0);
        assert_eq!(player.get_name(), "Test");
    }

    #[test]
    fn test_get_hand() {
        let player = Player::new("Test".to_string(), vec![], 0.0, 0, 0);
        assert_eq!(player.get_hand().len(), 0);
    }

    #[test]
    fn test_add_card_to_hand() {
        let mut player = Player::new("Test".to_string(), vec![], 0.0, 0, 0);
        assert_eq!(player.get_hand().len(), 0);
        let card = Card::new("As".to_string(), vec![1], "A".to_string());
        player.add_card_to_hand(card);
        assert_eq!(player.get_hand().len(), 1);
    }

    #[test]
    fn test_clear_hand() {
        let mut player = Player::new("Test".to_string(), vec![], 0.0, 0, 0);
        assert_eq!(player.get_hand().len(), 0);
        let card = Card::new("As".to_string(), vec![1], "A".to_string());
        player.add_card_to_hand(card);
        assert_eq!(player.get_hand().len(), 1);
        player.clear_hand();
        assert_eq!(player.get_hand().len(), 0);
    }

    #[test]
    fn test_get_balance() {
        let player = Player::new("Test".to_string(), vec![], 0.0, 0, 0);
        assert_eq!(player.get_balance(), 0.0);
    }

    #[test]
    fn test_deposit() {
        let mut player = Player::new("Test".to_string(), vec![], 0.0, 0, 0);
        assert_eq!(player.get_balance(), 0.0);
        player.deposit(100.0);
        assert_eq!(player.get_balance(), 100.0);
    }

    #[test]
    fn test_withdraw() {
        let mut player = Player::new("Test".to_string(), vec![], 0.0, 0, 0);
        player.deposit(100.0);
        player.withdraw(10.0);
        assert_eq!(player.get_balance(), 90.0);
    }

    #[test]
    fn test_get_wins() {
        let player = Player::new("Test".to_string(), vec![], 0.0, 0, 0);
        assert_eq!(player.get_wins(), 0);
    }

    #[test]
    fn test_add_win() {
        let mut player = Player::new("Test".to_string(), vec![], 0.0, 0, 0);
        assert_eq!(player.get_wins(), 0);
        player.add_win();
        assert_eq!(player.get_wins(), 1);
    }

    #[test]
    fn test_get_defeats() {
        let player = Player::new("Test".to_string(), vec![], 0.0, 0, 0);
        assert_eq!(player.get_defeats(), 0);
    }

    #[test]
    fn test_add_defeats() {
        let mut player = Player::new("Test".to_string(), vec![], 0.0, 0, 0);
        assert_eq!(player.get_defeats(), 0);
        player.add_defeats();
        assert_eq!(player.get_defeats(), 1);
    }
}