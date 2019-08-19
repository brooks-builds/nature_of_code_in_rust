use ggez::{ContextBuilder, Context, GameResult, graphics, event, nalgebra};
use ggez::event::{EventHandler};

struct Game {

}

impl Game {
    pub fn new(_context: &mut Context) -> Game {
        Game {}
    }
}

impl EventHandler for Game {
    fn update(&mut self, _context: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult<()> {
        graphics::clear(context, graphics::WHITE);
        let location = nalgebra::Point2::new(0.0, 0.0);
        let circle = graphics::Mesh::new_circle(context, graphics::DrawMode::fill(), location, 50.0, 1.0, graphics::Color::new(1.0, 0.0, 0.0, 5.0))?;

        graphics::draw(context, &circle, (nalgebra::Point2::new(100.0, 100.0),))?;

        graphics::present(context)
    }
}

fn main() {
    let (mut context, mut event_loop) = ContextBuilder::new("traditional_random_walk", "Brookzerker")
        .build()
        .expect("problem getting GGEZ context");

    let mut game = Game::new(&mut context);

    match event::run(&mut context, &mut event_loop, &mut game) {
        Ok(_) => println!("Exited cleanly"),
        Err(error) => println!("Error running game: {}", error)
    }
}
