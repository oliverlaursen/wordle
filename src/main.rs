use std::io::{Write, stdout};

use crate::game::Game;

pub mod game;

fn main() {
    let mut game = Game::new(5,3);
    println!("Du har {} forsøg til at gætte ordet",game.tries);

    loop {
        print!("Indtast dit gæt: ");
        stdout().flush().unwrap();
        let mut guess = String::new();
        std::io::stdin().read_line(&mut guess).unwrap();
        match game.guess(guess.trim()) {
            game::RoundResult::Won => {
                println!("Tillykke, du gættede rigtigt!");
                break;
            }
            game::RoundResult::Lost(str) => {
                println!("{}                  {} forsøg tilbage", str, game.tries);
                println!("Øv, du har ikke flere forsøg. Det rigtige ord var {}", game.word);
                break;
            }
            game::RoundResult::Continue(str) => println!("{}                  {} forsøg tilbage", str, game.tries),
            game::RoundResult::WrongLength => println!("Word must be 5 letters long"),
        }
    }
}
