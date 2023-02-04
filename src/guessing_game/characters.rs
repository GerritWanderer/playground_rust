use std::io;
use rand::Rng;

const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyz";

#[derive(Debug)]
pub struct Game {
    pub secret: char,
    pub guess: Option<char>,
}

impl Game {
  // constructor method to return a new Game with a random charactor set as secret
  pub fn build() -> Self {
    let i = rand::thread_rng().gen_range(0..CHARSET.len());
      Game {
          secret: char::from(CHARSET[i]),
          guess: None
      }
  }
}

impl super::GameTrait for Game {
    fn ask(&mut self) {
        let mut my_guess = String::new();
        io::stdin().read_line(&mut my_guess).expect("some io::stdin error happened");
        self.guess = match my_guess.trim().parse() {
            Ok(char) => Some(char),
            Err(_) => None,
        };
    }

    fn compare(&self) -> Result<bool, &str> {
        if self.secret == self.guess.expect("Guss cannot be determined") {
            Ok(true)
        } else {
            Err("wrong character")
        }
    }
}