mod players;
mod cards;
mod bot;

use std::io::*;
use players::players::Player;
use bot::bot::Bot;

struct BlackJack { }

impl BlackJack {
    pub fn check_balance(game: bool, player: Player) -> bool {
        if player.balance == 0 as f32 {
            println!("{}, you haven't balance", player.name);
            game == true;
        }

        game
    }

    pub fn bet(player: Player, computer: Bot) {
        let check = false;
        let mut bet_player: f32;
        let bet_computer: f32;
    }
}