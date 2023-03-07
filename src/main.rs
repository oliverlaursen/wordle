use std::{io::{Write, stdout}, fs, collections::HashMap};
use clap::Parser;
use crate::game::Game;

pub mod game;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, default_value_t = 6)]
    tries: u32,

    #[arg(short, default_value_t = 5)]
    word_length: usize,

    #[arg(short,long, default_value = "english")]
    language: String,
}

fn clear(){
    println!("{}[2J", 27 as char);
}

fn load_languages(language:String) -> HashMap<String, String>{
    let mut map:HashMap<String, String> = HashMap::new();
    let file = fs::read_to_string(format!("./languages/{}.txt",language));
    for line in file.unwrap().lines() {
        let key = line.split(":").next().unwrap().to_string();
        let value = line.split(":").nth(1).unwrap().to_string();
        map.insert(key, value);
    }
    map
}

fn main() {
    let args = Args::parse();
    let mut game = Game::new(args.word_length,args.tries,args.language.clone());
    let lang_map = load_languages(args.language);
    clear();
    println!("{} {} {}\n", lang_map.get("TRIES1").unwrap(),game.tries, lang_map.get("TRIES2").unwrap());


    loop {
        print!("{}: ",lang_map.get("GUESS").unwrap());
        stdout().flush().unwrap();
        let mut guess = String::new();
        std::io::stdin().read_line(&mut guess).unwrap();
        println!("");
        match game.guess(guess.trim()) {
            game::RoundResult::Won => {
                println!("{}",lang_map.get("YOU_WON").unwrap());
                break;
            }
            game::RoundResult::Lost => {
                clear();
                println!("{}                {} {}", game.prev_guesses, game.tries,lang_map.get("TRIES_LEFT").unwrap());
                println!("==================================================");
                game.print_alphabet();
                println!("{} {}", lang_map.get("LOSE").unwrap(),game.word);
                break;
            }
            game::RoundResult::Continue => {
                clear();
                println!("{}                {} {}", game.prev_guesses, game.tries,lang_map.get("TRIES_LEFT").unwrap());
                println!("==================================================");
                game.print_alphabet();
        },
            game::RoundResult::WrongLength => println!("{} {} {}",lang_map.get("WRONG_LENGTH1").unwrap(),args.word_length,lang_map.get("WRONG_LENGTH2").unwrap()),
            game::RoundResult::NotRealWord => println!("{}",lang_map.get("NOT_REAL_WORD").unwrap())
        }
    }
}
