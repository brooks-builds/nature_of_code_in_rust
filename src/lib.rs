use ggez::event::EventHandler;
use ggez::graphics::{self, Color, DrawParam, MeshBuilder, BLACK};
use ggez::{timer, Context, GameResult};
use mover::Mover;
use utilities::vector2::Vector2;

use crate::zone::Zone;

mod mover;
mod utilities;
mod zone;

pub struct MainState {
    background_color: Color,
    mover: Mover,
    zones: Vec<Zone>,
}

impl MainState {
    pub fn new(context: &mut Context) -> GameResult<Self> {
        let background_color = BLACK;
        let (width, height) = graphics::drawable_size(context);
        let mut mover = Mover::new(Vector2::new(0.0, height / 2.0), 25.0);
        mover.apply_force(Vector2::new(3.2, 0.0));
        let zones = vec![
            Zone::new(
                width / 2.0 - 500.0,
                0.0,
                500.0,
                height,
                Color::new(1.0, 0.0, 0.0, 0.3),
                0.01,
            ),
            Zone::new(
                width / 2.0,
                0.0,
                500.0,
                height,
                Color::new(0.0, 0.3, 0.7, 0.3),
                -0.01,
            ),
        ];

        Ok(Self {
            background_color,
            mover,
            zones,
        })
    }
}

impl EventHandler for MainState {
    fn update(&mut self, context: &mut Context) -> GameResult {
        let (width, _height) = graphics::drawable_size(context);
        while timer::check_update_time(context, 60) {
            for zone in &self.zones {
                zone.apply_friction_to_mover(&mut self.mover);
            }
            self.mover.update();
            self.mover.check_edges(width);
        }
        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
        graphics::clear(context, self.background_color);
        let mut mesh_builder = MeshBuilder::new();
        for zone in &self.zones {
            zone.draw(&mut mesh_builder);
        }
        self.mover.draw(&mut mesh_builder);
        let mesh = mesh_builder.build(context)?;
        graphics::draw(context, &mesh, DrawParam::new())?;

        graphics::present(context)
    }
}
