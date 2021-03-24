use ggez::event::EventHandler;
use ggez::graphics::{self, Color, DrawParam, Mesh, MeshBuilder, BLACK, WHITE};
use ggez::{Context, GameResult};
use mint::Point2;
use rand::prelude::ThreadRng;
use rand::{thread_rng, Rng};

pub struct MainState {
    background_color: Color,
    mesh: Option<Mesh>,
    rng: ThreadRng,
    points: Vec<Point2<f32>>,
    max_step: f32,
    min_step: f32,
}

impl MainState {
    pub fn new(context: &mut Context) -> GameResult<Self> {
        let background_color = BLACK;
        let rng = thread_rng();
        let (width, height) = graphics::drawable_size(context);
        let mesh = None;
        let points = vec![Point2 {
            x: width / 2.0,
            y: height / 2.0,
        }];
        let max_step = 50.0;
        let min_step = 1.0;

        Ok(Self {
            background_color,
            rng,
            mesh,
            points,
            max_step,
            min_step,
        })
    }

    /// Not sure if this is really exponential random number generator.
    fn random_exponential(&mut self) -> f32 {
        loop {
            let random_1 = self.rng.gen_range(self.min_step..self.max_step);
            let probability = random_1;
            let random_2 = self.rng.gen_range(self.min_step..self.max_step);
            if random_2 < probability / 4.0 {
                return random_1;
            }
        }
    }
}

impl EventHandler for MainState {
    fn update(&mut self, context: &mut Context) -> GameResult {
        let random_direction = self.rng.gen_range(0..4);
        let last_point = self.points.last().unwrap();
        let mut next_point = *last_point;
        if random_direction == 0 {
            next_point.y -= self.random_exponential();
        } else if random_direction == 1 {
            next_point.x += self.random_exponential();
        } else if random_direction == 2 {
            next_point.y += self.random_exponential();
        } else {
            next_point.x -= self.random_exponential();
        }
        self.points.push(next_point);

        let mesh = MeshBuilder::new()
            .line(&self.points, 2.0, WHITE)?
            .build(context)?;

        self.mesh = Some(mesh);

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
