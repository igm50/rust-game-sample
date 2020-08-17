mod image;
mod state;

use ggez::*;
use state::MainState;
use std::boxed::Box;

fn main() {
    let resource_dir = std::path::Path::new("./resources");

    let c = conf::Conf::new();
    let (ref mut ctx, ref mut event_loop) = ContextBuilder::new("sounder", "jintz")
        .add_resource_path(resource_dir)
        .conf(c)
        .build()
        .unwrap();

    let image = image::Image::new(
        graphics::Image::new(ctx, "/janken_gu.png").unwrap(),
        300.0,
        100.0,
    );
    let draw_items: Vec<Box<dyn state::DrawItem>> = vec![Box::from(image)];
    let ref mut state = MainState::new(ctx, draw_items).unwrap();

    event::run(ctx, event_loop, state).unwrap();
}
