use ggez::{ContextBuilder, Context, event, GameResult, graphics};
use ggez::event::{EventHandler};

struct Game {

}

impl Game {
    pub fn new(_context: &mut Context) -> Game {
        // store a vector (20 length) with 0.0 in each
        // [1.0, 1.0, 2.0]
        Game {}
    }
}

impl EventHandler for Game {
    fn update(&mut self, _context: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult<()> {
        // loop through the stored vector and draw a rectangle for each with height being the value
        graphics::present(context)
    }
}

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
