use std::io;

pub mod numbers;
pub mod characters;

// return an instance of either number or character game

pub trait GameTrait {
    fn ask(&mut self) -> ();
    fn compare(&self) -> Result<bool, &str>;
}

pub fn start() -> Box<dyn GameTrait> {
    let mut game_choice = String::new();
    println!("which type do you want to play? Enter 1 for a numbers game and 2 for a characters game");
    loop {
        io::stdin().read_line(&mut game_choice).expect("some io::stdin error happened");
        let game: Box<dyn GameTrait> = match game_choice.trim().parse::<u32>() {
            Ok(num) => {
                if num == 1 {
                    println!("Initializing a numbers game. Let's type in your guess (0-9)");
                    Box::new(numbers::Game::build())
                } else {
                    println!("Initializing a characters game. Let's type in your guess (a-z)");
                    Box::new(characters::Game::build())
                }
            },
            Err(_) => continue,
        };
        break game
    }
}

