mod players;
mod cards;
mod bot;

use std::io;
use players::players::Player;
use bot::bot::Bot;

struct BlackJack { }

impl BlackJack {
    pub fn check_balance(player: Player) -> bool {
        if player.balance == 0 as f32 {
            println!("{}, you haven't balance", player.name);
            true
        } else {
            false
        }
    }

    pub fn bet(player: &mut Player, computer: &mut Bot) -> [f32; 2] {
        let mut bet_player: f32 = 0.0;
        let bet_computer: f32;

        while bet_player <= 0.0 {
            let mut capture_bet_player: String = String::new();
            io::stdin().read_line(&mut capture_bet_player).unwrap_or_default();
            bet_player = capture_bet_player.parse::<f32>().unwrap_or_default();
        }

        player.balance -= bet_player;

        if bet_player > computer.balance {
            println!("{}, I haven't this money, I'll give all win, right?", player.name);
            bet_computer = computer.balance;
            computer.balance = 0.0;
        } else {
            bet_computer = bet_player;
            computer.balance -= bet_computer;
        }

        [bet_player, bet_computer]
    }

    pub fn check_play_again(choice: &str) -> bool {
        if choice.to_uppercase() == "Y" {
            false
        } else {
            true
        }
    }
}