use ecosystem::Game;

fn main() {
    let mut game = Game::new();

    bbggez::run(&mut game, "Ecosystem Project", "Brookzerker");
}