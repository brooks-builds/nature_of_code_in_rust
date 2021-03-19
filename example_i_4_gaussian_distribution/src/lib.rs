use ggez::event::EventHandler;
use ggez::graphics::{self, Color, DrawMode, DrawParam, Mesh, MeshBuilder};
use ggez::mint::Point2;
use ggez::{timer, Context, GameResult};
use rand::prelude::ThreadRng;
use rand::thread_rng;
use rand_distr::{Distribution, Normal};

pub struct MainState {
    y: f32,
    x: f32,
    mesh: Mesh,
    rng: ThreadRng,
    random: Normal<f32>,
}

impl MainState {
    pub fn new(context: &mut Context) -> GameResult<Self> {
        let (width, height) = graphics::drawable_size(context);
        let y = height / 2.0;
        let x = width / 2.0;
        let mesh = MeshBuilder::new()
            .circle(
                DrawMode::fill(),
                Point2 { x: 0.0, y: 0.0 },
                10.0,
                0.1,
                Color::new(1.0, 1.0, 1.0, 0.01),
            )
            .build(context)?;
        let rng = thread_rng();
        let random = Normal::new(width / 2.0, 100.0).unwrap();
        Ok(Self {
            y,
            x,
            mesh,
            rng,
            random,
        })
    }
}

impl EventHandler for MainState {
    fn update(&mut self, context: &mut Context) -> GameResult {
        while timer::check_update_time(context, 60) {
            self.x = self.random.sample(&mut self.rng);
        }

        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
        graphics::draw(context, &self.mesh, DrawParam::new().dest([self.x, self.y]))?;
        graphics::present(context)
    }
}
