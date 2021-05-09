use ggez::event::EventHandler;
use ggez::graphics::{self, Color, DrawMode, DrawParam, Mesh, MeshBuilder, BLACK, WHITE};
use ggez::mint::Point2;
use ggez::{Context, GameResult};
use rand::prelude::ThreadRng;
use rand::thread_rng;
use rand_distr::{Distribution, Normal};

pub struct MainState {
    locations: Vec<Point2<f32>>,
    colors: Vec<Color>,
    circle: Mesh,
    location_rng: Normal<f32>,
    color_rng: Normal<f32>,
    rng: ThreadRng,
}

impl MainState {
    pub fn new(context: &mut Context) -> GameResult<Self> {
        let circle = MeshBuilder::new()
            .circle(DrawMode::fill(), Point2 { x: 0.0, y: 0.0 }, 5.0, 0.1, WHITE)
            .build(context)?;
        let location_rng = Normal::new(500.0, 250.0).unwrap();
        let color_rng = Normal::new(200.0, 20.0).unwrap();
        let rng = thread_rng();
        let locations = vec![];
        let colors = vec![];
        Ok(Self {
            locations,
            colors,
            circle,
            location_rng,
            color_rng,
            rng,
        })
    }
}

impl EventHandler for MainState {
    fn update(&mut self, _context: &mut Context) -> GameResult {
        let point = Point2 {
            x: self.location_rng.sample(&mut self.rng),
            y: self.location_rng.sample(&mut self.rng),
        };
        self.locations.push(point);
        let color = Color::from_rgb(
            self.color_rng.sample(&mut self.rng) as u8,
            self.color_rng.sample(&mut self.rng) as u8,
            self.color_rng.sample(&mut self.rng) as u8,
        );
        self.colors.push(color);
        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
        graphics::clear(context, BLACK);
        for (index, location) in self.locations.iter().enumerate() {
            graphics::draw(
                context,
                &self.circle,
                DrawParam::new().dest(*location).color(self.colors[index]),
            )?;
        }
        graphics::present(context)
    }
}
