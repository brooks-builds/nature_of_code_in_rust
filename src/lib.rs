use ggez::event::EventHandler;
use ggez::graphics::{self, Color, DrawParam, MeshBuilder, BLACK};
use ggez::{timer, Context, GameResult};
use mover::Mover;
use rand::{thread_rng, Rng};
use utilities::vector2::Vector2;
use zone::Zone;

mod mover;
mod utilities;
mod zone;

pub struct MainState {
    movers: Vec<Mover>,
    gravity: Vector2,
    water: Zone,
}

impl MainState {
    pub fn new(context: &mut Context) -> GameResult<Self> {
        let mut rng = thread_rng();
        let (width, height) = graphics::drawable_size(context);
        let space_between = width / 10.0;
        let mut movers = vec![];
        let gravity = Vector2::new(0.0, 0.1);
        let water = Zone::new(
            0.0,
            height / 2.0,
            width,
            height / 2.0,
            Color::new(0.0, 0.0, 1.0, 0.3),
            0.5,
        );

        for count in 1..=10 {
            let mass = rng.gen_range(5.0..30.0);
            let mover = Mover::new(
                Vector2::new(count as f32 * space_between + mass, 30.0),
                mass,
            );
            movers.push(mover);
        }
        Ok(Self {
            movers,
            gravity,
            water,
        })
    }
}

impl EventHandler for MainState {
    fn update(&mut self, context: &mut Context) -> GameResult {
        let drawable_size = graphics::drawable_size(context);
        while timer::check_update_time(context, 60) {
            for mover in &mut self.movers {
                let mut gravity = self.gravity;
                gravity *= mover.mass;
                mover.apply_force(gravity);
                if mover.is_inside_zone(&self.water) {
                    mover.apply_drag(&self.water);
                }
                mover.update();
                mover.check_edges(drawable_size);
            }
        }
        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
        graphics::clear(context, BLACK);
        let mut mesh_builder = MeshBuilder::new();
        for mover in &self.movers {
            mover.draw(&mut mesh_builder);
        }
        self.water.draw(&mut mesh_builder);
        let mesh = mesh_builder.build(context)?;
        graphics::draw(context, &mesh, DrawParam::new())?;

        graphics::present(context)
    }
}
