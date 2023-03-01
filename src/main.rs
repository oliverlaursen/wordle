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

fn load_languages() -> HashMap<String, HashMap<String,String>>{
    let mut lang_map:HashMap<String, HashMap<String,String>> = HashMap::new();
    let langs = fs::read_dir("./languages").unwrap();
    for lang in langs{
        let file_name = lang.unwrap().file_name();
        let lang_name = file_name.to_str().unwrap().trim_end_matches(".txt").to_string();
        let mut map:HashMap<String,String> = HashMap::new();
        let file = fs::read_to_string(format!("./languages/{}",file_name.to_str().unwrap()));
        for line in file.unwrap().lines() {
            let key = line.split(":").next().unwrap().to_string();
            let value = line.split(":").nth(1).unwrap().to_string();
            map.insert(key, value);
        }
        lang_map.insert(lang_name, map);
    }
    lang_map
}

fn main() {
    let args = Args::parse();
    let mut game = Game::new(args.word_length,args.tries,args.language.clone());
    let lang_map = load_languages();
    let lang_map = lang_map.get(&args.language.to_string()).unwrap();
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
            game::RoundResult::Lost(str) => {
                clear();
                println!("{}                {} {}", game.prev_guesses, game.tries,lang_map.get("TRIES_LEFT").unwrap());
                println!("==================================================");
                game.print_alphabet();
                println!("{} {}", lang_map.get("LOSE").unwrap(),game.word);
                break;
            }
            game::RoundResult::Continue(_str) => {
                clear();
                println!("{}                {} {}", game.prev_guesses, game.tries,lang_map.get("TRIES_LEFT").unwrap());
                println!("==================================================");
                game.print_alphabet();
        },
            game::RoundResult::WrongLength => println!("{} {} {}",lang_map.get("WRONG_LENGTH1").unwrap(),args.word_length,lang_map.get("WRONG_LENGTH2").unwrap()),
        }
    }
}
