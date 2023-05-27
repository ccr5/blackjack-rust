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
use dealer::dealer::{check_winner};

use crate::deck::deck::Deck;
use crate::human::human::Human;

pub struct BlackJack { }

struct Bets {
    player_bet_amount: f32,
    computer_bet_amount: f32
}

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
        let balance = checked_balance.trim_end().parse::<f32>().unwrap();
        let mut human: Human = Human::new(name, vec![], balance, 0, 0);

        let mut bot: String = String::new();
        println!("Could you choose a name for a bot? ");
        io::stdin().read_line(&mut bot).unwrap();
        let mut computer = Bot::new(bot, vec![], balance, 0, 0);
        computer.welcome_message();

        println!("So let's play? ");
        let mut deck = Deck::new();
        let mut game = false;

        while game == false {
            deck.create_deck();
            deck.shuffle_deck();
            let check_balance_player: bool = self.check_balance(&human.player);
            let check_balance_computer: bool = self.check_balance(&computer.player);

            if check_balance_player | check_balance_computer {
                game = true;
                continue;
            }

            let bets = self.bet(&mut human.player, &mut computer.player);
            
            let mut ask_player = false;

            while !ask_player {
    
                if human.player.get_hand().len() == 0 {
                    println!("You don't have any card in your hand")
                } else {
                    human.player.show_hand();
                }
                
                let mut more_card: String = String::new();
                println!("Would you like one more card? ");
                io::stdin().read_line(&mut more_card).unwrap();
    
                if more_card.trim_end().to_uppercase() == "Y" {
                    human.player.add_card_to_hand(deck.get_card().unwrap());
                    ask_player = false;
                } else {
                    ask_player = true;
                }
            }
    
            let mut bot_player = false;
    
            while !bot_player {
                let bot_action = computer.play_game();
                if bot_action {
                    computer.player.add_card_to_hand(deck.get_card().unwrap());
                } else {
                    bot_player = true;
                }
            }
    
            let (winner, total_player, total_computer) = match check_winner(human.player.get_hand(), computer.player.get_hand()) {
                Some(val) => val,
                None => panic!("Error to check winner")
            };

            self.check_result(
                &mut human.player,
                &mut computer.player,
                bets,
                &winner,
                total_player,
                total_computer
            );

            human.player.show_info();
            human.player.show_hand();

            computer.player.show_info();
            computer.player.show_hand();

            let mut choice = String::new();
            println!("Do you wanna play again (Y/N): ");
            io::stdin().read_line(&mut choice).unwrap();
            game = self.check_play_again(&choice.trim_end().to_uppercase());

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

    fn bet<T: PlayerType>(&self, player: &mut T, computer: &mut T) -> Bets {
        let mut bet_player: f32 = 0.0;
        let bet_computer: f32;

        while bet_player <= 0.0 {
            let mut capture_bet_player: String = String::new();
            println!("Insert how many will you bet in this match: ");
            io::stdin().read_line(&mut capture_bet_player).unwrap_or_default();
            bet_player = capture_bet_player.trim_end().parse::<f32>().unwrap_or_default();
            
            if bet_player > player.get_balance() {
                println!("You haven't balance enough to bet this amount")
            }
        }

        player.withdraw(bet_player);

        if bet_player > computer.get_balance() {
            println!("{}, I haven't this money, I'll give all win, right?", player.get_name());
            bet_computer = computer.get_balance();
            computer.withdraw(bet_computer);
        } else {
            bet_computer = bet_player;
            computer.withdraw(bet_computer);
        }

        Bets { player_bet_amount: bet_player, computer_bet_amount: bet_computer }
    }

    fn check_result<T: PlayerType>(
        &self, 
        player: &mut T,
        computer: &mut T,
        bets: Bets,
        winner: &String,
        total_player: i8,
        total_computer: i8
    ) {
        if winner == "no win" {
            println!("no one win, you guys have more than 21!");
            println!("{}: {} \t{}: {}", player.get_name(), total_player, computer.get_name(), total_computer);
            player.deposit(bets.player_bet_amount);
            computer.deposit(bets.computer_bet_amount);
        } else if winner == "draw" {
            println!("it's a draw O.o");
            println!("{}: {} \t{}: {}", player.get_name(), total_player, computer.get_name(), total_computer);
            player.deposit(bets.player_bet_amount);
            computer.deposit(bets.computer_bet_amount);
            player.add_win();
            computer.add_win();
        } else if winner == "h1" {
            println!("{} win !!", player.get_name());
            println!("{}: {} \t{}: {}", player.get_name(), total_player, computer.get_name(), total_computer);
            player.deposit(bets.player_bet_amount + bets.computer_bet_amount);
            player.add_win();
            computer.add_defeats();
        } else {
            println!("{} win !!", computer.get_name());
            println!("{}: {} \t{}: {}", player.get_name(), total_player, computer.get_name(), total_computer);

            computer.deposit(bets.player_bet_amount + bets.computer_bet_amount);
            computer.add_win();
            player.add_defeats();
        }   
    }

    fn check_play_again(&self, choice: &String) -> bool {
        if choice.to_uppercase() == "Y" {
            false
        } else {
            true
        }
    }
}