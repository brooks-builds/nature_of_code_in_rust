use bbggez::run;
use ecosystem::Game;

fn main() {
    let width = 1900.0;
    let height = 1000.0;
    let mut game = Game::new(width, height);

    run::run_dim(
        &mut game,
        width,
        height,
        "Nature of Code Ecosystem Project",
        "Brookzerker",
    );
}
