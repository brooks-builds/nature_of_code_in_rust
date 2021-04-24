use ggez::event::EventHandler;
use ggez::graphics::{self, Color, BLACK};
use ggez::{Context, GameResult};
use mover::Mover;
use rand::{thread_rng, Rng};

mod mover;
pub mod utilities;

pub struct MainState {
    background_color: Color,
    movers: Vec<Mover>,
}

impl MainState {
    pub fn new(context: &mut Context) -> GameResult<Self> {
        let background_color = BLACK;
        let (width, height) = graphics::drawable_size(context);
        let mut rng = thread_rng();
        let mut movers = vec![];
        for _ in 0..20 {
            let x = rng.gen_range(0.0..width);
            let y = rng.gen_range(0.0..height);
            let mover = Mover::new(x, y, context)?;
            movers.push(mover);
        }

        Ok(Self {
            background_color,
            movers,
        })
    }
}

impl EventHandler for MainState {
    fn update(&mut self, context: &mut Context) -> GameResult {
        // for mover in &mut self.movers {
        //     mover.walk(context);
        //     mover.update();
        //     mover.check_edges(context);
        // }
        self.movers.iter_mut().for_each(|mover| {
            mover.walk(context);
            mover.update();
            mover.check_edges(context);
        });
        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
        graphics::clear(context, self.background_color);
        // for mover in &mut self.movers {
        //     mover.display(context)?;
        // }
        self.movers
            .iter()
            .try_for_each(|mover| mover.display(context))?;
        graphics::present(context)
    }
}
