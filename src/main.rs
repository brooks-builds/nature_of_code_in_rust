use ggez::{event, ContextBuilder, GameResult};
use i_1_traditional_random_walk::MainState;

fn main() -> GameResult {
    let (context, event_loop) =
        &mut ContextBuilder::new("Traditional Random Walk", "Brooks Builds")
            .build()
            .unwrap();

    let mut main_state = MainState::new(context)?;

    event::run(context, event_loop, &mut main_state)
}
