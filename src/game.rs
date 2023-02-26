use std::fs;
use rand::{seq::SliceRandom};

pub struct Game{
    pub tries:u32,
    word_length:usize,
    word:String,
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

    pub fn guess(&mut self,guess:&str) -> String {
        let mut result = String::new();
        let mut correct = 0;
        if guess.chars().count()!=self.word_length{
            return String::from("Word must be 5 letters long");
        }

        for (index,character) in guess.chars().enumerate(){
            if self.word.chars().nth(index).unwrap()==character{ 
                result.push_str(format!("\u{001b}[32m{}",character).as_str()); // Green
                correct+=1;
            }
            else if self.word.contains(character){
                result.push_str(format!("\u{001b}[33m{}",character).as_str()); // Yellow
            }
            else{
                result.push_str(format!("\u{001b}[0m{}",character).as_str());
            }
        }
        result.push_str("\u{001b}[0m"); // Resets color
        self.tries-=1;
        if self.tries==0 {
            self.loser();
        }
        if correct==self.word_length {
            self.tries=0;
            self.winner();
        }
        result
    }

    fn winner(&self){
        println!("Tilykke, du gættede rigtigt!")
    }

    fn loser(&self){
        println!("Øv, du har ikke flere forsøg, det rigtige ord var {}",self.word);
    }
}