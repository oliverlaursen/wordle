use std::{fs, collections::HashMap};
use rand::seq::SliceRandom;
use ansi_term::{Colour};

pub struct Game{
    pub tries: u32,
    pub word: String,
    alphabet:Vec<WordleChar>,
}

#[derive(Clone,Debug)]
pub struct WordleChar{
    pub c: char,
    pub state: WordleCharState,
}

#[derive(Clone,Copy,Debug)]
pub enum WordleCharState{
    Neutral,
    Correct,
    SemiCorrect,
    NotUsed,
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
            .map(|c| WordleChar{c,state:WordleCharState::Neutral})
            .collect::<Vec<WordleChar>>();
        Self {
            tries,
            word,
            alphabet,
        }
    }

    pub fn print_alphabet(&self){
        let alph = self.alphabet.clone();
        println!("{} \n",self.wordle_chars_to_string(alph));
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
            let mut guess:Vec<WordleChar> = guess.chars().map(|x| WordleChar{c:x,state:WordleCharState::Neutral}).collect();
            for (i,c) in guess.iter_mut().enumerate(){
                if c.c==self.word.as_bytes()[i] as char && char_counts.get(&c.c).unwrap()>&0{
                    c.state=WordleCharState::Correct;
                    if let Some(count)=char_counts.get_mut(&c.c){
                        *count-=1;
                    }
                    self.update_char(c.clone(),WordleCharState::Correct);
                }
                else{
                    self.update_char(c.clone(), WordleCharState::NotUsed);
                }
            }
            for c in guess.iter_mut(){
                if self.word.contains(c.c) && char_counts.get(&c.c).unwrap()>&0 && !matches!(c.state,WordleCharState::Correct){
                    c.state=WordleCharState::SemiCorrect;
                    if let Some(count)=char_counts.get_mut(&c.c){
                        *count-=1;
                    }
                    self.update_char(c.clone(), WordleCharState::SemiCorrect);
                }
            }
            let result_string = self.wordle_chars_to_string(guess);
            if self.tries == 0 {
                RoundResult::Lost(result_string)
            }
            else {
                RoundResult::Continue(result_string)
            }
            
        }
    }

    pub fn wordle_chars_to_string(&self,input:Vec<WordleChar>) -> String{
        let mut result="".to_string();
        for c in input{
            let painted = self.paint_wordle_char(c);
            result.push_str(&painted);
            result.push_str(" ");
        }
        result
    }

    pub fn paint_wordle_char(&self, input:WordleChar) -> String{
        let col = match input.state {
            WordleCharState::Neutral => Colour::White,
            WordleCharState::Correct => Colour::Green,
            WordleCharState::SemiCorrect => Colour::RGB(255, 140, 0),
            WordleCharState::NotUsed => Colour::RGB(128, 128, 128),
        };
        col.paint(input.c.to_string()).to_string()
    }

    pub fn update_char(&mut self, c:WordleChar, st: WordleCharState){
        let mut alph = self.alphabet.clone();
        for mut i in alph.iter_mut(){
            if c.c==i.c {
                if !matches!(i.state,WordleCharState::Correct){
                    i.state=st;
                }
            }
        }
        self.alphabet=alph;
    }
}
