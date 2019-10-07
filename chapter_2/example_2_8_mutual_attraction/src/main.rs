use bbggez::run;
use mutual_attraction::Game;

fn main() {
    let mut game = Game::new();

    run(&mut game, "Mutual Attraction", "Brookzerker");
}
