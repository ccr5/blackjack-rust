mod cards;
mod deck;
mod players;
mod bot;

use std::io::{stdin};
use players::players::Player;
use bot::bot::Bot;

use crate::deck::deck::Deck;


fn main() {
    let mut name: String = String::new();
    println!("Welcome to BlackJack -_-");

    println!("First, What is your name? ");
    stdin().read_line(&mut name).unwrap_or_default();

    println!("How many money have you today? ");
    let mut checked_balance: String = String::new();
    stdin().read_line(&mut checked_balance).unwrap_or_default();
    let balance = checked_balance.parse::<f32>().unwrap_or_default();

    let player = Player::new(name, vec![], balance, 0, 0);

    let mut bot: String = String::new();
    println!("Could you choose a name for a bot? ");
    stdin().read_line(&mut bot).unwrap_or_default();
    let computer = Bot::new(bot, vec![], balance, 0, 0);
    computer.welcome_message();

    println!("So let's play? ");
    let deck = Deck::new();

    let game = false;

    while game == false {
        let game_deck = deck.create_deck();
    }


}
