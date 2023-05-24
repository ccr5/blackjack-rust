pub mod players {
    pub struct Player {
        name: String,
        hand: Vec<Card>,
        balance: f32,
        wins: i32,
        defeats: i32
    }

    impl  Player {

        pub fn new(name: String, hand: Vec<Card>, balance: f32, wins: i32, defeats: i32) -> Self {
            Player { name, hand, balance, wins, defeats }
        }

        pub fn show_info(&self) {
            match self.balance {
                Ordering::greeter(0) => println!("{}, your balance is: {}", self.name, self.balance),
                Ordering::equal(0) => println!("{}, you haven't balance, thanks for play BlackJack", self.name),
                _ => println!("{}, you're owe {}", self.name, self.balance)
            }
        }
    }
}