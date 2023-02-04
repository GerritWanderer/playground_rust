pub mod guessing_game;

fn main() {
    // start() will return a new game by asking the user which game type should be used
    let mut game = guessing_game::start();

    // as long as the Result from compare is not <Ok>, ask() is called
    loop {
        game.ask();
        match game.compare() {
            Ok(_) => break,                  // Ok is returned when the users guess was correct
            Err(hint) => println!("{hint}"), // print a hint and repeat asking the user for a new guess
        }
    }
    // when the loop is over, the game is over too
    println!("Game finished. Congrats!");
}
