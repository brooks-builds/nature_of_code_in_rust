use ggez::event::EventHandler;
use ggez::graphics::{self, DrawMode, DrawParam, Mesh, MeshBuilder, WHITE};
use ggez::mint::Point2;
use ggez::{Context, GameResult};

pub struct MainState {
    x: f32,
    y: f32,
    mesh: Mesh,
}

impl MainState {
    pub fn new(context: &mut Context) -> GameResult<Self> {
        let x = 50.0;
        let y = 50.0;
        let mesh = MeshBuilder::new()
            .circle(DrawMode::fill(), Point2 { x: 0.0, y: 0.0 }, 1.0, 0.1, WHITE)
            .build(context)?;

        Ok(Self { x, y, mesh })
    }
}

impl EventHandler for MainState {
    fn update(&mut self, _context: &mut Context) -> GameResult {
        let random_number = rand::random::<f32>();

        if random_number < 0.2 {
            self.y -= 1.0;
        } else if random_number < 0.4 {
            self.x -= 1.0;
        } else if random_number < 0.7 {
            self.x += 1.0;
        } else {
            self.y += 1.0;
        }

        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
        graphics::draw(context, &self.mesh, DrawParam::new().dest([self.x, self.y]))?;
        graphics::present(context)
    }
}
