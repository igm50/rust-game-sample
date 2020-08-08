mod state;

use ggez::*;
use state::GameState;

pub fn main() {
    let mut state = GameState::new(std::time::Duration::new(0, 0));

    let c = conf::Conf::new();
    let (ref mut ctx, ref mut event_loop) = ContextBuilder::new("sounder", "jintz")
        .conf(c)
        .build()
        .unwrap();

    event::run(ctx, event_loop, &mut state).unwrap();
}
