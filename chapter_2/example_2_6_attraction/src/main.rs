use attraction::*;
use bbggez::{
    ggez,
    ggez::{ContextBuilder, event},
    ggez::conf::Conf
};

fn main() {
    let mut conf = Conf::new();
    conf.window_mode = conf.window_mode.resizable(true).dimensions(1860.0/2.0, 1015.0);
    conf.window_setup = conf.window_setup.title("Attraction");
    let (mut context, mut event_loop) = ContextBuilder::new("Attraction", "Brookzerker")
        .conf(conf)
        .build()
        .expect("Game context was not able to be created");
    let mut game = Game::new(&mut context);

    match event::run(&mut context, &mut event_loop, &mut game) {
        Ok(_) => println!("Exited cleanly"),
        Err(error) => println!("Error occured: {}", error),
    };
}