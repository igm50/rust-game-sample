use ggez::event::EventHandler;
use ggez::*;
use std::boxed::Box;

pub trait DrawItem {
  fn draw(&self, ctx: &mut Context) -> GameResult;
}

pub struct MainState {
  draw_items: Vec<Box<dyn DrawItem>>,
}

impl MainState {
  pub fn new(_ctx: &mut Context, draw_items: Vec<Box<dyn DrawItem>>) -> GameResult<MainState> {
    Ok(MainState {
      draw_items: draw_items,
    })
  }
}

impl EventHandler for MainState {
  fn update(&mut self, _ctx: &mut Context) -> GameResult {
    Ok(())
  }

  fn draw(&mut self, ctx: &mut Context) -> GameResult {
    graphics::clear(ctx, graphics::Color::new(1.0, 1.0, 1.0, 0.0));

    for di in &self.draw_items {
      di.draw(ctx)?;
    }

    graphics::present(ctx)
  }
}
