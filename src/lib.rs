use ggez::event::EventHandler;
use ggez::graphics::{self, DrawMode, DrawParam, Mesh, MeshBuilder, WHITE};
use ggez::input::mouse;
use ggez::mint::Point2;
use ggez::{Context, GameResult};

pub struct MainState {
    x: f32,
    y: f32,
    mesh: Mesh,
}

impl MainState {
    pub fn new(context: &mut Context) -> GameResult<Self> {
        let (width, height) = graphics::drawable_size(context);
        let x = width / 2.0;
        let y = height / 2.0;
        let mesh = MeshBuilder::new()
            .circle(DrawMode::fill(), Point2 { x: 0.0, y: 0.0 }, 1.0, 0.1, WHITE)
            .build(context)?;

        Ok(Self { x, y, mesh })
    }

    fn move_towards_mouse(&mut self, context: &mut Context) {
        let mouse_position = mouse::position(context);

        if self.x < mouse_position.x {
            self.x += 1.0;
        } else if self.x > mouse_position.x {
            self.x -= 1.0;
        }

        if self.y < mouse_position.y {
            self.y += 1.0;
        } else if self.y > mouse_position.y {
            self.y -= 1.0;
        }
    }

    fn randomly_move(&mut self) {
        let random_number = rand::random::<f32>();

        if random_number < 0.25 {
            self.y -= 1.0;
        } else if random_number < 0.5 {
            self.y += 1.0;
        } else if random_number < 0.75 {
            self.x += 1.0;
        } else {
            self.x -= 1.0;
        }
    }
}

impl EventHandler for MainState {
    fn update(&mut self, context: &mut Context) -> GameResult {
        if rand::random::<f32>() < 0.5 {
            self.move_towards_mouse(context);
        } else {
            self.randomly_move();
        }
        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
        graphics::draw(context, &self.mesh, DrawParam::new().dest([self.x, self.y]))?;
        graphics::present(context)
    }
}
