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
    coefficient_of_friction: f32,
}

impl MainState {
    pub fn new(context: &mut Context) -> GameResult<Self> {
        let background_color = BLACK;
        let wind_force = Vector2::new(0.1, 0.0);
        let gravity_force = Vector2::new(0.0, 0.3);
        let mut random = Random::new();
        let mut movers = vec![];

        for _ in 0..100 {
            let mass = random.range(1.0, 5.0);
            let mover = Mover::new(0.0, 0.0, mass, context)?;
            movers.push(mover);
        }

        Ok(Self {
            background_color,
            movers,
            wind_force,
            gravity_force,
            coefficient_of_friction: 0.04,
        })
    }
}

impl EventHandler for MainState {
    fn update(&mut self, context: &mut Context) -> GameResult {
        let wind_force = &self.wind_force;

        let gravity_force = self.gravity_force;
        let coefficient_of_friction = self.coefficient_of_friction;
        self.movers.iter_mut().for_each(|mover| {
            let mut gravity_force = gravity_force;
            let mut friction = mover.velocity;

            friction *= -1.0;
            friction.normalize();
            friction *= coefficient_of_friction;

            gravity_force *= mover.mass;

            mover.apply_force(wind_force);
            mover.apply_force(&gravity_force);
            mover.apply_force(&friction);

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
