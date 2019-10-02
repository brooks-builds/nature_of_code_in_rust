use fluid::*;
use ggez::{ContextBuilder, event};
use ggez::conf::Conf;

fn main() {
    let mut conf = Conf::new();
    conf.window_mode = conf.window_mode
        .dimensions(1860.0/2.0, 1015.0)
        .resizable(true);
    conf.window_setup = conf.window_setup.title("Fluid Resistance");
    let (mut context, mut event_loop) = ContextBuilder::new("Fluid Resistance", "Brookzerker")
        .conf(conf)
        .build()
        .expect("Game context was not able to be created");
    let mut game = Game::new(&mut context);

    match event::run(&mut context, &mut event_loop, &mut game) {
        Ok(_) => println!("Exited cleanly"),
        Err(error) => println!("Error occured: {}", error),
    };
}