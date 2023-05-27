use blackjack_rust::BlackJack;
use std::process;

fn main() {
    let bj: BlackJack = BlackJack::new();
    if let Err(e) = bj.run() {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
