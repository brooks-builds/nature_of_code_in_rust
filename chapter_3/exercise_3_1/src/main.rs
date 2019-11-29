use bbggez::run::run;
use exercise_3_1::Game;

fn main() {
    let mut game = Game::new();

    run(&mut game, "Rotate Baton-Like Object", "Brookzerker");
}
