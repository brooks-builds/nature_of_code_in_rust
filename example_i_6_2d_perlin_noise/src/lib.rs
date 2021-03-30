use ggez::event::EventHandler;
use ggez::graphics::{self, Color, DrawMode, DrawParam, MeshBuilder};
use ggez::{Context, GameResult};
use noise::{NoiseFn, Perlin};
use utilities::map;

mod utilities;

pub struct MainState {}

impl MainState {
    pub fn new(context: &mut Context) -> GameResult<Self> {
        let (width, height) = graphics::drawable_size(context);
        let mut mesh_builder = MeshBuilder::new();

        let noise = Perlin::new();
        let noise_incrementer = 0.01;

        let mut xoff = 10_000.0;
        for x in 0..width as u32 {
            let mut yoff = 0.0;
            for y in 0..height as u32 {
                let brightness = map(noise.get([xoff, yoff]) as f32, -1.0, 1.0, 0.0, 1.0);
                let color = Color::new(brightness, brightness, brightness, 1.0);
                mesh_builder.circle(DrawMode::fill(), [x as f32, y as f32], 0.5, 0.1, color);
                yoff += noise_incrementer;
            }
            xoff += noise_incrementer;
        }

        let mesh = mesh_builder.build(context)?;

        graphics::draw(context, &mesh, DrawParam::new())?;
        graphics::present(context)?;
        Ok(Self {})
    }
}

impl EventHandler for MainState {
    fn update(&mut self, _context: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, _context: &mut Context) -> GameResult {
        Ok(())
    }
}
