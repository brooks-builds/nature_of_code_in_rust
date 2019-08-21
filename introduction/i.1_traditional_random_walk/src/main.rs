use ggez::{ContextBuilder, Context, GameResult, graphics, event, nalgebra};
use ggez::event::{EventHandler};
use rand::{thread_rng, Rng};

struct Game {
    location: nalgebra::Point2<f32>,
    radius: f32,
}

impl Game {
    pub fn new(context: &mut Context) -> Game {
        let (width, height) = graphics::drawable_size(context);

        Game {
            location: nalgebra::Point2::new(width / 2.0, height / 2.0),
            radius: 1.0
        }
    }
}

impl EventHandler for Game {
    fn update(&mut self, _context: &mut Context) -> GameResult<()> {
        let mut rng = thread_rng();
        let direction = rng.gen_range(0, 4);

        match direction {
            0 => self.location.x = self.location.x + 1.0,
            1 => self.location.y = self.location.y + 1.0,
            2 => self.location.x = self.location.x - 1.0,
            3 => self.location.y = self.location.y - 1.0,
            _ => unreachable!("unexpected random number given"),
        };

        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult<()> {
        let circle = graphics::Mesh::new_circle(
            context, 
            graphics::DrawMode::fill(), 
            nalgebra::Point2::new(0.0, 0.0), 
            self.radius, 
            0.1, 
            graphics::WHITE
        )?;

        graphics::draw(context, &circle, (self.location,))?;

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
