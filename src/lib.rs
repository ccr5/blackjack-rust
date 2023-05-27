mod cards;
mod deck;
mod players;
mod bot;
mod human;
mod dealer;

use std::io;
use std::error::Error;
use bot::bot::Bot;
use players::players::PlayerType;

use crate::deck::deck::Deck;
use crate::human::human::Human;

pub struct BlackJack { }

impl BlackJack {

    pub fn new() -> Self { BlackJack {  } }

    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        let mut name: String = String::new();
        println!("Welcome to BlackJack -_-");

        println!("First, What is your name? ");
        io::stdin().read_line(&mut name).unwrap_or_default();

        println!("How many money have you today? ");
        let mut checked_balance: String = String::new();
        io::stdin().read_line(&mut checked_balance).unwrap_or_default();
        let balance = checked_balance.parse::<f32>().unwrap_or_default();
        let human: Human = Human::new(name, vec![], balance, 0, 0);

        let mut bot: String = String::new();
        println!("Could you choose a name for a bot? ");
        io::stdin().read_line(&mut bot).unwrap_or_default();
        let computer = Bot::new(bot, vec![], balance, 0, 0);
        computer.welcome_message();

        println!("So let's play? ");
        let mut deck = Deck::new();
        let bj: BlackJack = BlackJack::new();

        let mut game = false;

        while game == false {
            deck.create_deck();
            deck.shuffle_deck();
            let check_balance_player: bool = bj.check_balance(&human.player);
            let check_balance_computer: bool = bj.check_balance(&computer.player);

            if check_balance_player | check_balance_computer {
                game = true;
                continue;
            }
        }

        Ok(())
    }

    fn check_balance<T: PlayerType>(&self, player: &T) -> bool {
        if player.get_balance() == 0 as f32 {
            println!("{}, you haven't balance", player.get_name());
            true
        } else {
            false
        }
    }

    fn bet<T: PlayerType>(&self, player: &mut T, computer: &mut T) -> [f32; 2] {
        let mut bet_player: f32 = 0.0;
        let bet_computer: f32;

        while bet_player <= 0.0 {
            let mut capture_bet_player: String = String::new();
            io::stdin().read_line(&mut capture_bet_player).unwrap_or_default();
            bet_player = capture_bet_player.parse::<f32>().unwrap_or_default();
        }

        player.withdraw(bet_player);

        if bet_player > computer.get_balance() {
            println!("{}, I haven't this money, I'll give all win, right?", player.get_name());
            bet_computer = computer.get_balance();
            computer.set_balance(0.0);
        } else {
            bet_computer = bet_player;
            computer.withdraw(bet_computer);
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