use bbggez::ggez::{conf::Conf, event, ContextBuilder};
use ecosystem::Game;

fn main() {
    let width = 1900.0;
    let height = 1000.0;
    let mut conf = Conf::new();

    conf.window_mode = conf.window_mode.resizable(false).dimensions(width, height);

    conf.window_setup = conf
        .window_setup
        .title("Nature of Code Ecosystem Project")
        .vsync(false);

    let (mut context, mut event_loop) = ContextBuilder::new("", "Brookzerker")
        .conf(conf)
        .build()
        .expect("Game context was not able to be created");

    let mut game = Game::new(width, height, &mut context);

    match event::run(&mut context, &mut event_loop, &mut game) {
        Ok(_) => println!("Exited cleanly"),
        Err(error) => println!("Error occured: {}", error),
    };
}
