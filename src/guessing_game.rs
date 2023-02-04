use std::io;

// the guessing game supports two types of games, which are nested in it's own sub-module
pub mod numbers; // the user should guess a number from 0-9
pub mod characters; // the user should guess a charactnr from a-z

// The GameTrait contains a common interfaces, which both game types share
pub trait GameTrait {
    fn ask(&mut self) -> ();
    fn compare(&self) -> Result<bool, &str>;
}

pub fn start() -> Box<dyn GameTrait> {
    println!("which game type do you want to play?");
    println!("Enter 1 for a numbers game and 2 for a characters game");
    let mut game_choice = String::new();
    
    loop {
        // open a prompt to the user, the input from the user is bound to game_choice
        io::stdin().read_line(&mut game_choice).expect("some io::stdin error happened");

        // use a condition to check which game type was selected
        // return the correct struct by calling the constructor method build()
        let result: Result<Box<dyn GameTrait>, &str> = match game_choice.trim().parse::<u32>() {
            Ok(num) => {
                if num == 1 {
                    println!("Initializing a numbers game. Let's type in your guess (0-9)");
                    Ok(Box::new(numbers::Game::build()))
                } else if num == 2 {
                    println!("Initializing a characters game. Let's type in your guess (a-z)");
                    Ok(Box::new(characters::Game::build()))
                } else {
                    Err("Your choice does not exist, please choose between 1 and 2")
                }
            },
            Err(_) => continue,
        };

        // stop the loop and return the game when the previous scope was successful
        // otherwise print the error message and start again
        match result {
            Ok(game) => break game,
            Err(message) => {
                println!("{message}");
                continue
            }
        }
    }
}

