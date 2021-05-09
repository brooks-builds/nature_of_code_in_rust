use ggez::event::EventHandler;
use ggez::graphics::{self, Color, DrawMode, DrawParam, Mesh, MeshBuilder, WHITE};
use ggez::{Context, GameResult};
use noise::{NoiseFn, Perlin};
use utilities::map;

mod utilities;

pub struct MainState {
    pixel_mesh: Mesh,
    noise: Perlin,
    noise_incrementer: f64,
    zoff: f64,
}

impl MainState {
    pub fn new(context: &mut Context) -> GameResult<Self> {
        let pixel_mesh = MeshBuilder::new()
            .circle(DrawMode::fill(), [0.0, 0.0], 0.5, 0.1, WHITE)
            .build(context)?;
        let noise = Perlin::new();
        let noise_incrementer = 0.01;
        Ok(Self {
            pixel_mesh,
            noise_incrementer,
            noise,
            zoff: 0.0,
        })
    }
}

impl EventHandler for MainState {
    fn update(&mut self, _context: &mut Context) -> GameResult {
        self.zoff += self.noise_incrementer;
        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
        let (width, height) = graphics::drawable_size(context);

        let mut xoff = 10_000.0;
        for x in 0..width as u32 {
            let mut yoff = 0.0;
            for y in 0..height as u32 {
                let brightness = map(
                    self.noise.get([xoff, yoff, self.zoff]) as f32,
                    -1.0,
                    1.0,
                    0.0,
                    1.0,
                );
                let color = Color::new(brightness, brightness, brightness, 1.0);
                graphics::draw(
                    context,
                    &self.pixel_mesh,
                    DrawParam::new().dest([x as f32, y as f32]).color(color),
                )?;
                yoff += self.noise_incrementer;
            }
            xoff += self.noise_incrementer;
        }
        graphics::present(context)?;
        Ok(())
    }
}
