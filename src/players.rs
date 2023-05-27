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
        fn get_balance(&self) -> f32;
        fn set_balance(&mut self, new_balance: f32) -> bool;
        fn withdraw(&mut self, value: f32) -> bool;
        fn get_wins(&self) -> i32;
        fn add_win(&mut self) -> bool;
        fn get_defeats(&self) -> i32;
        fn add_defeats(&mut self) -> bool;
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

        fn get_balance(&self) -> f32 {
            self.balance
        }

        fn set_balance(&mut self, new_balance: f32) -> bool {
            self.balance = new_balance;
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