use std::io::{self, Write};

use crate::game::Game;
use std::io::stdout;

pub mod game;

fn main() {
    let mut game = Game::new();
    println!("Du har {} forsøg til at gætte ordet",game.tries);

    while game.tries > 0 {
        print!("Enter your guess :");
        let mut guess = String::new();
        std::io::stdin().read_line(&mut guess).unwrap();
        println!("{}",game.guess(guess.trim()));
        print!("\u{001b}[0m");  // Resets color
    }
}
