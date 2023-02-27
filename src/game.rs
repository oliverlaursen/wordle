use std::fs;
use rand::seq::SliceRandom;
use ansi_term::Colour;

pub struct Game{
    pub tries: u32,
    word_length:usize,
    pub word: String,
}

pub enum RoundResult {
    Continue(String),
    Won,
    Lost(String),
    WrongLength,
}

impl Game{

    pub fn generate_word(word_length:usize) -> String {
        let word_list:String = fs::read_to_string("wordlist_danish.txt").unwrap();
        let word_list_fixed_length:Vec<&str> = word_list
            .split_whitespace()
            .filter(|word| word.len() == word_length)
            .collect();
        let mut rng = rand::thread_rng();
        let word = word_list_fixed_length.choose(&mut rng).unwrap();
        return word.to_string();
    }

    pub fn new(word_length:usize,tries:u32) -> Game{
        let w = Game::generate_word(word_length);
        Game {tries:tries,word_length:word_length,word:w}
    }

    pub fn guess(&mut self,guess:&str) -> RoundResult {
        self.tries -= 1;
        if guess.len() != self.word.len() {
            self.tries += 1;
            RoundResult::WrongLength
        }
        else if guess == self.word {
            RoundResult::Won
        }
        else {
            let result = guess.chars()
                .zip(self.word.chars())
                .map(|(a, b)| {
                    if a == b {
                        Colour::Green.paint(a.to_string()).to_string()
                    }
                    else if self.word.contains(a) {
                        Colour::Yellow.paint(a.to_string()).to_string()
                    }
                    else {
                        a.to_string()
                    }
                })
                .collect();
            if self.tries == 0 {
                RoundResult::Lost(result)
            }
            else {
                RoundResult::Continue(result)
            }
        }
    }
}
