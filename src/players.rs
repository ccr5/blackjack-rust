pub mod players {
    
    use crate::cards::cards::Card;

    pub struct Player {
        pub name: String,
        pub hand: Vec<Card>,
        pub balance: f32,
        pub wins: i32,
        pub defeats: i32
    }

    impl  Player {

        pub fn new(name: String, hand: Vec<Card>, balance: f32, wins: i32, defeats: i32) -> Self {
            Player { name, hand, balance, wins, defeats }
        }

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