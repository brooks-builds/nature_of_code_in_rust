use ggez::event::EventHandler;
use ggez::graphics::{self, Color, DrawParam, Mesh, MeshBuilder, BLACK, WHITE};
use ggez::mint::Point2;
use ggez::{Context, GameResult};
use rand::prelude::ThreadRng;
use rand::thread_rng;
use rand_distr::{Distribution, Normal};

pub struct MainState {
    background_color: Color,
    steps: Vec<Point2<f32>>,
    rng: ThreadRng,
    step_rng: Normal<f32>,
    mesh: Option<Mesh>,
}

impl MainState {
    pub fn new(context: &mut Context) -> GameResult<Self> {
        let background_color = BLACK;
        let mut steps = vec![];
        let (width, height) = graphics::drawable_size(context);
        let x = width / 2.0;
        let y = height / 2.0;
        steps.push(Point2 { x, y });
        let rng = thread_rng();
        let step_rng = Normal::new(0.0, 10.0).unwrap();
        let mesh = None;

        Ok(Self {
            background_color,
            steps,
            rng,
            step_rng,
            mesh,
        })
    }
}

impl EventHandler for MainState {
    fn update(&mut self, context: &mut Context) -> GameResult {
        let last_location = self.steps.last().unwrap();
        let next_x = self.step_rng.sample(&mut self.rng) + last_location.x;
        let next_y = self.step_rng.sample(&mut self.rng) + last_location.y;
        self.steps.push(Point2 {
            x: next_x,
            y: next_y,
        });
        self.mesh = Some(
            MeshBuilder::new()
                .line(&self.steps, 2.0, WHITE)?
                .build(context)?,
        );
        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
        graphics::clear(context, self.background_color);
        if let Some(mesh) = &self.mesh {
            graphics::draw(context, mesh, DrawParam::new())?;
        }
        graphics::present(context)
    }
}
