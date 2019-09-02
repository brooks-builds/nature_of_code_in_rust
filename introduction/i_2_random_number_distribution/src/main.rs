use i_2_random_number_distribution::*;
use ggez::{ContextBuilder, event};

fn main() {
    let (mut context, mut event_loop) = ContextBuilder::new("random_distribution", "Brookzerker")
        .build()
        .expect("Game context was not able to be created");

    let mut game = Game::new(&mut context);

    match event::run(&mut context, &mut event_loop, &mut game) {
        Ok(_) => println!("Exited cleanly"),
        Err(error) => println!("Error occured: {}", error),
    };
}
