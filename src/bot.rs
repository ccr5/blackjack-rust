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

        pub fn play_game(&self) -> bool {
            if self.player.get_hand().len() == 0 {
                true
            } else {
                let mut count_a = 0;
                let mut sum_hand = 0;

                for card in self.player.get_hand() {
                    if card.get_ticker() == "A" {
                        count_a += 1;

                        if count_a.ge(&1) && sum_hand.gt(&10) {
                            sum_hand += 1 * count_a;
                        } else  {
                            sum_hand += 11;
                        }

                    } else {
                        sum_hand += card.get_height()[0];
                    }
                }
                    

            sum_hand < 21
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{bot::bot::Bot, cards::cards::Card};

    #[test]
    fn test_play_game() {
        let bot = Bot::new("Test".to_string(), vec![], 0.0, 0, 0);
        assert_eq!(bot.play_game(), true);

        let card_a = Card::new("As".to_string(), vec![1], "A".to_string());
        let bot = Bot::new("Test".to_string(), vec![card_a], 0.0, 0, 0);
        assert_eq!(bot.play_game(), true);

        let card_a = Card::new("As".to_string(), vec![1], "A".to_string());
        let card_ten = Card::new("Ten".to_string(), vec![10], "10".to_string());
        let bot = Bot::new("Test".to_string(), vec![card_a, card_ten], 0.0, 0, 0);
        assert_eq!(bot.play_game(), false);

        let card_eight = Card::new("Eight".to_string(), vec![8], "8".to_string());
        let card_ten = Card::new("Ten".to_string(), vec![10], "10".to_string());
        let bot = Bot::new("Test".to_string(), vec![card_eight, card_ten], 0.0, 0, 0);
        assert_eq!(bot.play_game(), true);
    }

}