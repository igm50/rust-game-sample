// mod image;
mod phase;
mod state;

use ggez::*;
use state::MainState;

fn main() {
    let resource_dir = std::path::Path::new("./resources");

    let conf = conf::Conf::new();
    let (ref mut ctx, ref mut event_loop) = ContextBuilder::new("sounder", "jintz")
        .add_resource_path(resource_dir)
        .conf(conf)
        .build()
        .unwrap();

    let ref mut state = MainState::new(ctx).unwrap();

    event::run(ctx, event_loop, state).unwrap();
}
