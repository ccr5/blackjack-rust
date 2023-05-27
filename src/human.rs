pub mod human {
    use crate::{
        players::players::{Player, PlayerType},
        cards::cards::Card
    };

    pub struct Human {
        pub player: Player
    }

    impl Human {
        pub fn new(name: String, hand: Vec<Card>, balance: f32, wins: i32, defeats: i32) -> Self {
            Human { player: Player::new(name, hand, balance, wins, defeats) }
        }
    }

}