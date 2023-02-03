pub mod guessing_game;

fn main() {
    let mut game = guessing_game::start();
    loop {
        game.ask();
        match game.compare() {
            Ok(_) => break,
            Err(hint) => {
                println!("{hint}");
            }
        }
    };
    println!("Game finished. Congrats!");
}