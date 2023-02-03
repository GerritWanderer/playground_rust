use std::io;
use rand::Rng;

const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyz";

#[derive(Debug)]
pub struct Game {
    pub secret: char,
    pub guess: char,
    pub history: Vec<u32>,
}

impl super::GameTrait for Game {
  fn ask(&mut self) {
      let mut my_guess = String::new();
      io::stdin().read_line(&mut my_guess).expect("my input was wrond");
      self.guess = match my_guess.trim().parse() {
          Ok(char) => char,
          Err(_) => 'x',
      };
  }

  fn compare(&self) -> Result<bool, &str> {
    if self.secret == self.guess {
      Ok(true)
    } else {
      Err("wrong character")
    }
  }
}

impl Game {
  pub fn build() -> Self {
    let i = rand::thread_rng().gen_range(0..CHARSET.len());
      Game {
          secret: char::from(CHARSET[i]),
          guess: 'x',
          history: Vec::new()
      }
  }
}