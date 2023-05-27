mod cards;
mod deck;
mod players;
mod bot;

use std::io;
use players::players::Player;
use bot::bot::Bot;

use crate::deck::deck::Deck;
use rand::seq::SliceRandom;

pub struct BlackJack { }

impl BlackJack {
    pub fn new() -> Self {
        BlackJack {  }
    }

    pub fn run(&self) {
        let mut name: String = String::new();
        println!("Welcome to BlackJack -_-");

        println!("First, What is your name? ");
        io::stdin().read_line(&mut name).unwrap_or_default();

        println!("How many money have you today? ");
        let mut checked_balance: String = String::new();
        io::stdin().read_line(&mut checked_balance).unwrap_or_default();
        let balance = checked_balance.parse::<f32>().unwrap_or_default();

        let player: Player = Player::new(name, vec![], balance, 0, 0);

        let mut bot: String = String::new();
        println!("Could you choose a name for a bot? ");
        io::stdin().read_line(&mut bot).unwrap_or_default();
        let computer = Bot::new(bot, vec![], balance, 0, 0);
        computer.welcome_message();

        println!("So let's play? ");
        let deck = Deck::new();
        let bj: BlackJack = BlackJack::new();

        let game = false;

        while game == false {
            let mut game_deck = deck.create_deck();
            game_deck.shuffle(&mut rand::thread_rng());
            let check_balance_player: bool = bj.check_balance(&player);
        }
    }

    fn check_balance(&self, player: &Player) -> bool {
        if player.balance == 0 as f32 {
            println!("{}, you haven't balance", player.name);
            true
        } else {
            false
        }
    }

    fn bet(&self, player: &mut Player, computer: &mut Bot) -> [f32; 2] {
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

    fn check_play_again(&self, choice: &str) -> bool {
        if choice.to_uppercase() == "Y" {
            false
        } else {
            true
        }
    }
}