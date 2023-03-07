use std::{fs, collections::HashMap};
use rand::seq::SliceRandom;
use ansi_term::{Colour};

pub struct Game{
    pub tries: u32,
    pub word: String,
    alphabet:Vec<WordleChar>,
    pub prev_guesses:String,
    pub language:String,
    pub full_wordlist:Vec<String>,
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
    Continue,
    Won,
    Lost,
    WrongLength,
    NotRealWord,
}

impl Game{
    pub fn generate_word(word_length: usize,language: String) -> String {
        let word_list:String = fs::read_to_string(format!("./wordlists/wordlist_{language}.txt",))
            .expect("Could not find wordlist"); 

        let word_list_fixed_length:Vec<&str> = word_list
          .lines().filter(|word| word.chars().count() == word_length)
          .collect();
        let mut rng = rand::thread_rng();
        let word = word_list_fixed_length.choose(&mut rng).unwrap();
        word.to_uppercase()
    }

    pub fn new(word_length: usize, tries: u32, language: String) -> Self {
        let word = Game::generate_word(word_length,language.clone());
        let prev_guesses = String::new();
        let alphabet = match language.as_str(){
            "danish" => "QWERTYUIOPÅASDFGHJKLÆØZXCVBNM",
            _ => "QWERTYUIOPASDFGHJKLZXCVBNM",
        };
        let alphabet = alphabet.chars()
            .map(|c| WordleChar{c,state:WordleCharState::Neutral})
            .collect::<Vec<WordleChar>>();
        let full_wordlist:Vec<String> = fs::read_to_string(format!("./wordlists/wordlistfull_{language}.txt",))
            .expect("Could not find wordlistfull")
            .lines()
            .map(|x|x.to_uppercase())
            .collect(); 
        Self {
            tries,
            word,
            alphabet,
            prev_guesses,
            language,
            full_wordlist,
        }
    }

    pub fn print_alphabet(&self){
        let alph = self.alphabet.clone();
        let key_offsets:Vec<usize>;
        if self.language=="danish"{
            key_offsets=vec![10,21,28]
        }
        else{
            key_offsets=vec![9,18,25]
        }
        
        let row1 = alph[0..=key_offsets[0]].to_vec();
        let row2 = alph[key_offsets[0]+1..=key_offsets[1]].to_vec();
        let row3 = alph[key_offsets[1]+1..=key_offsets[2]].to_vec();



        println!("{}",self.wordle_chars_to_string(row1));
        println!("{}",self.wordle_chars_to_string(row2));
        println!("{} \n",self.wordle_chars_to_string(row3));


    }

    pub fn guess(&mut self, guess:&str) -> RoundResult {
        self.tries -= 1;
        let guess = guess.to_uppercase();

        // Create char_counts
        let mut char_counts = HashMap::new();
        for c in self.word.chars() {
            *char_counts.entry(c).or_insert(0) += 1;}

        if guess.chars().count() != self.word.chars().count() {
            self.tries += 1;
            return RoundResult::WrongLength;
        }
        else if guess == self.word {
            RoundResult::Won
        }
        else if !self.full_wordlist.contains(&guess){
            RoundResult::NotRealWord
        }
        else {
            // Convert guess chars to wordlechars
            let mut guess:Vec<WordleChar> = guess.chars().map(|x| WordleChar{c:x,state:WordleCharState::Neutral}).collect();
            
            // Check for chars with correct placement (green)
            for (i,c) in guess.iter_mut().enumerate(){
                if c.c==self.word.chars().nth(i).unwrap() as char && char_counts.get(&c.c).unwrap()>&0{
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

            // Check for semi correct (orange) chars
            for c in guess.iter_mut(){
                if self.word.contains(c.c) && char_counts.get(&c.c).unwrap()>&0 && !matches!(c.state,WordleCharState::Correct){
                    c.state=WordleCharState::SemiCorrect;
                    if let Some(count)=char_counts.get_mut(&c.c){*count-=1;}
                    self.update_char(c.clone(), WordleCharState::SemiCorrect);
                }
            }
            let result_string = self.wordle_chars_to_string_centered(guess);
            self.prev_guesses.push_str(result_string.as_str());
            self.prev_guesses.push_str("\n");
            if self.tries == 0 {
                RoundResult::Lost
            }
            else {
                RoundResult::Continue
            }
        }
    }

    pub fn wordle_chars_to_string(&self,input:Vec<WordleChar>) -> String{
        let mut result="".to_string();
        result.push_str("┌───┐".repeat(input.len()).as_str());
        result.push_str("\n");
        for c in input.clone(){
            let painted = self.paint_wordle_char(c);
            result.push_str("│ ");
            result.push_str(&painted);
            result.push_str(" │");
        }
        result.push_str("\n");
        result.push_str("└───┘".repeat(input.len()).as_str());
        result
    }

    pub fn wordle_chars_to_string_centered(&self,input:Vec<WordleChar>) -> String{
        let mut result="".to_string();
        result.push_str("           ");
        result.push_str("┌───┐".repeat(input.len()).as_str());
        result.push_str("\n           ");
        for c in input.clone(){
            let painted = self.paint_wordle_char(c);
            result.push_str("│ ");
            result.push_str(&painted);
            result.push_str(" │");
        }
        result.push_str("\n           ");
        result.push_str("└───┘".repeat(input.len()).as_str());
        result
    }

    pub fn paint_wordle_char(&self, input:WordleChar) -> String{
        let col = match input.state {
            WordleCharState::Neutral => Colour::White,
            WordleCharState::Correct => Colour::Green,
            WordleCharState::SemiCorrect => Colour::RGB(255, 140, 0),
            WordleCharState::NotUsed => Colour::RGB(70, 70, 70),
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
