pub struct Game{
    word:String,
    pub tries:u32,
}

impl Game{
    pub fn new() -> Game{
        let w = String::from("kager");
        Game {word:w,tries:3}
    }

    pub fn guess(&mut self,guess:String) -> String {
        let mut result = String::new();
        if guess.chars().count()!=5{
            return String::from("Word must be 5 letters long");
        }

        for (index,character) in guess.chars().enumerate(){
            if self.word.chars().nth(index).unwrap()==character{
                result.push_str(format!("\u{001b}[32m{}",character).as_str());
            }
            else if self.word.contains(character){
                result.push_str(format!("\u{001b}[33m{}",character).as_str());
            }
            else{
                result.push_str(format!("\u{001b}[0m{}",character).as_str());
            }
        }
        print!("\u{001b}[0m");
        self.tries-=1;
        result
    }
}