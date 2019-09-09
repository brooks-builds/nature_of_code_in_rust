use exercise_1_8_velocity_with_acceleration::*;
use ggez::{ContextBuilder, event};

fn main() {
    let (mut context, mut event_loop) = ContextBuilder::new("game_name", "Brookzerker")
        .build()
        .expect("Game context was not able to be created");

    let mut game = Game::new(&mut context);

    match event::run(&mut context, &mut event_loop, &mut game) {
        Ok(_) => println!("Exited cleanly"),
        Err(error) => println!("Error occured: {}", error),
    };
}