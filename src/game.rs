pub struct Game{
    word:String,
    pub tries:u32,
}

impl Game{
    pub fn new() -> Game{
        let w = String::from("kager");
        Game {word:w,tries:3}
    }

    pub fn guess(&mut self,guess:&str) -> String {
        let mut result = String::new();
        let mut correct = 0;
        if guess.chars().count()!=5{
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
        if correct==5 {
            self.tries=0;
        }
        result
    }
}