use crate::game::Game;

pub mod game;

fn main() {
    let mut game = Game::new();
    println!("Du har {} forsøg til at gætte ordet",game.tries);

    while game.tries > 0 {
        print!("Enter your guess :");
        let mut guess = String::new();
        let _b1 = std::io::stdin().read_line(&mut guess).unwrap();
        println!("{}",game.guess(guess));
    }
}
