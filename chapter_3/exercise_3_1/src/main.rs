use bbggez::ggez::{event, ContextBuilder};
use exercise_3_1::Game;

fn main() {
    // Make a Context.
    let (mut context, mut event_loop) =
        ContextBuilder::new("Angular motion using rotation", "Brookzerker")
            .build()
            .expect("aieee, could not create ggez context!");

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let mut my_game = Game::new(&mut context).unwrap();

    // Run!
    match event::run(&mut context, &mut event_loop, &mut my_game) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e),
    }
}
