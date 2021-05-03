use ggez::event::EventHandler;
use ggez::graphics::{self, Color, BLACK};
use ggez::{Context, GameResult};
use mover::Mover;
use utilities::random::Random;
use utilities::vector2::Vector2;

mod mover;
pub mod utilities;

pub struct MainState {
    background_color: Color,
    movers: Vec<Mover>,
    wind_force: Vector2,
    gravity_force: Vector2,
}

impl MainState {
    pub fn new(context: &mut Context) -> GameResult<Self> {
        let background_color = BLACK;
        let wind_force = Vector2::new(0.1, 0.0);
        let gravity_force = Vector2::new(0.0, 1.0);
        let mut random = Random::new();
        let mut movers = vec![];

        for _ in 0..100 {
            let mass = random.range(10.0, 35.0);
            let mover = Mover::new(mass * 2.0, mass * 2.0, mass, context)?;
            movers.push(mover);
        }

        Ok(Self {
            background_color,
            movers,
            wind_force,
            gravity_force,
        })
    }
}

impl EventHandler for MainState {
    fn update(&mut self, context: &mut Context) -> GameResult {
        let wind_force = &self.wind_force;
        let gravity_force = &self.gravity_force;

        self.movers.iter_mut().for_each(|mover| {
            mover.apply_force(wind_force);
            mover.apply_force(gravity_force);
            mover.update();
            mover.check_edges(context);
        });

        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
        graphics::clear(context, self.background_color);
        self.movers
            .iter()
            .try_for_each(|mover| mover.display(context))?;
        graphics::present(context)
    }
}
