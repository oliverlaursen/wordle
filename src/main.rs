use std::io::{Write, stdout};

use crate::game::Game;

pub mod game;

fn main() {
    let mut game = Game::new();
    println!("Du har {} forsøg til at gætte ordet",game.tries);

    while game.tries > 0 {
        print!("Indtast dit gæt: ");
        stdout().flush().unwrap();
        let mut guess = String::new();
        std::io::stdin().read_line(&mut guess).unwrap();
        println!("{}                  {} forsøg tilbage",game.guess(guess.trim()),game.tries);
    }
}
