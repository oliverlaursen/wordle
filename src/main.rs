use std::io::{Write, stdout};
use clap::Parser;
use crate::game::Game;

pub mod game;

#[derive(Parser,Debug)]
struct Args {
    #[arg(short, default_value_t = 6)]
    tries:u32,

    #[arg(short, default_value_t = 5)]
    word_length:usize,
}

fn main() {
    let args = Args::parse();
    let mut game = Game::new(args.word_length,args.tries);
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
            game::RoundResult::Continue(str) => {
                println!("{}                  {} forsøg tilbage", str, game.tries);
                game.print_alphabet();
        },
            game::RoundResult::WrongLength => println!("Word must be 5 letters long"),
        }
    }
}
