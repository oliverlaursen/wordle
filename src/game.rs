use std::{fs, collections::HashMap};
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
        println!("\n");
    }

    pub fn guess(&mut self, guess:&str) -> RoundResult {
        self.tries -= 1;
        let guess = guess.to_uppercase();
        let mut char_counts = HashMap::new();
        for c in self.word.chars() {
            *char_counts.entry(c).or_insert(0) += 1;
        }

        if guess.chars().count() != self.word.chars().count() {
            self.tries += 1;
            return RoundResult::WrongLength;
        }
        else if guess == self.word {
            RoundResult::Won
        }
        else {
            let mut result:String = guess.chars()
                .zip(self.word.clone().chars())
                .map(|(a, b)| {
                    if a == b && char_counts.get(&a).unwrap()>&0 {
                        self.alphabet=self.update_char(a, Colour::Green);
                        if let Some(count) = char_counts.get_mut(&a) {
                            *count -= 1;
                        }
                        Colour::Green.paint(a.to_string()).to_string()
                    }
                    else {
                        self.alphabet=self.update_char(a, Colour::RGB(128,128,128));
                        a.to_string()
                    }
                })
                .collect();
            for (i,c) in guess.chars().enumerate() {
                let green = self.word.as_bytes()[i]==guess.as_bytes()[i];
                if self.word.contains(c) && !green && *char_counts.get(&c).unwrap()>0 {
                    self.alphabet=self.update_char(c, Colour::RGB(255, 140, 0));
                    result = result.clone().replace(c, Colour::RGB(255, 140, 0).paint(c.to_string()).to_string().as_str());
                }
            }
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
            let char_uncolored = i.chars().filter(|x|x.is_uppercase()).last().unwrap().to_string();
            let uncolored = i.chars().count()==1;
            let green = i.starts_with("\u{1b}[32");
            if char_uncolored==c.to_uppercase().to_string() {
                if uncolored || !green{
                    *i=col.paint(char_uncolored).to_string();
                }
            }
        }
        return result;
    }
}
