use std::fs;
use rand::seq::SliceRandom;
use ansi_term::Colour;

pub struct Game{
    pub tries: u32,
    pub word: String,
}

pub enum RoundResult {
    Continue(String),
    Won,
    Lost(String),
    WrongLength,
}

impl Game{
    pub fn generate_word(word_length: usize) -> String {
        let word_list:String = fs::read_to_string("wordlist_danish.txt").unwrap();
        let word_list_fixed_length:Vec<&str> = word_list
          .lines().filter(|word| word.chars().count() == word_length)
          .collect();
        let mut rng = rand::thread_rng();
        let word = word_list_fixed_length.choose(&mut rng).unwrap();
        word.to_string()
    }

    pub fn new(word_length: usize, tries: u32) -> Self {
        let word = Game::generate_word(word_length);
        Self {
            tries,
            word,
        }
    }

    pub fn guess(&mut self,guess:&str) -> RoundResult {
        self.tries -= 1;
        if guess.chars().count() != self.word.len() {
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
