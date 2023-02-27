use std::fs;
use rand::seq::SliceRandom;
use ansi_term::{Colour};

pub struct Game{
    pub tries: u32,
    pub word: String,
    alphabet:Vec<String>,
}

pub enum RoundResult {
    Continue(String),
    Won,
    Lost(String),
    WrongLength,
}


impl Game{
    pub fn generate_word(word_length: usize) -> String {
        let word_list:String = fs::read_to_string("popular_wordlist_danish.txt").unwrap(); 
        let word_list_fixed_length:Vec<&str> = word_list
          .lines().filter(|word| word.chars().count() == word_length)
          .collect();
        let mut rng = rand::thread_rng();
        let word = word_list_fixed_length.choose(&mut rng).unwrap();
        word.to_uppercase()
    }

    pub fn new(word_length: usize, tries: u32) -> Self {
        let word = Game::generate_word(word_length);
        let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZÆØÅ".chars()
            .map(|c| c.to_string())
            .collect::<Vec<String>>();
        Self {
            tries,
            word,
            alphabet,
        }

    }

    pub fn print_alphabet(&self){
        for i in &self.alphabet{
            print!("{} ",i);
        }
        println!("");
    }

    pub fn guess(&mut self, guess:&str) -> RoundResult {
        self.tries -= 1;
        let guess = guess.to_uppercase();
        if guess.chars().count() != self.word.chars().count() {
            self.tries += 1;
            return RoundResult::WrongLength;
        }
        else if guess == self.word {
            RoundResult::Won
        }
        else {
            let result = guess.chars()
                .zip(self.word.clone().chars())
                .map(|(a, b)| {
                    if a == b {
                        self.alphabet=self.update_char(a, Colour::Green);
                        Colour::Green.paint(a.to_string()).to_string()
                    }
                    else if self.word.contains(a) {
                        self.alphabet=self.update_char(a, Colour::RGB(255, 140, 0));
                        Colour::RGB(255, 140, 0).paint(a.to_string()).to_string()
                    }
                    else {
                        self.alphabet=self.update_char(a, Colour::RGB(128,128,128));
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

    pub fn update_char(&mut self, c:char,col:Colour) -> Vec<String>{
        let mut result = self.alphabet.clone();
        for i in &mut result{
            if i==&c.to_uppercase().to_string(){
                *i=col.paint(i.clone()).to_string();
            }
        }
        return result;
    }
}
