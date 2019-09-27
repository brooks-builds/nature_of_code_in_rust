use example_2_2_forces_acting_on_many_objects::*;
use ggez::{ContextBuilder, event};

fn main() {
    let (mut context, mut event_loop) = ContextBuilder::new("Forces acting on many objects", "Brookzerker")
        .build()
        .expect("Game context was not able to be created");

    let mut game = Game::new(&mut context);

    match event::run(&mut context, &mut event_loop, &mut game) {
        Ok(_) => println!("Exited cleanly"),
        Err(error) => println!("Error occured: {}", error),
    };
}