pub mod bot {

    use crate::{
        cards::cards::Card, 
        players::players::{Player, PlayerType}
    };

    pub struct Bot {
        pub player: Player
    }

    impl Bot {
        pub fn new(name: String, hand: Vec<Card>, balance: f32, wins: i32, defeats: i32) -> Self {
            Bot { player: Player::new(name, hand, balance, wins, defeats) }
        }

        pub fn welcome_message(&self) {
            println!("Hi o/ \nMy name is {} and I'll play with you", self.player.get_name());
            println!("Good luck!");
            println!("Aaah, I have the same balance for a fair play :)");
        }

        pub fn play_game(&self) {
            println!("Hellow!")
        }

    }
}