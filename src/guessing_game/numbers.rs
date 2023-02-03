use std::io;
use rand::Rng;
use std::cmp::Ordering;

#[derive(Debug)]
pub struct Game {
    pub secret: u32,
    pub guess: u32,
    pub history: Vec<u32>,
}

impl super::GameTrait for Game {
    fn ask(&mut self) {
        let mut my_guess = String::new();
        io::stdin().read_line(&mut my_guess).expect("my input was wrond");
        self.guess = match my_guess.trim().parse() {
            Ok(num) => num,
            Err(_) => 0,
        };
    }

    fn compare(&self) -> Result<bool, &str> {
        match self.secret.cmp(&self.guess) {
            Ordering::Less => { Err("lower") } 
            Ordering::Greater => { Err("higher") } 
            Ordering::Equal => Ok(true)
        }
    }
}

impl Game {
  pub fn build() -> Self {
      Game {
          secret: rand::thread_rng().gen_range(1..10),
          guess: 0,
          history: Vec::new()
      }
  }
}