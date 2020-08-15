use ggez::event::EventHandler;
use ggez::input::mouse::MouseButton;
use ggez::*;
use std::boxed::Box;

pub trait DrawItem {
  fn draw(&self, ctx: &mut Context) -> GameResult;
  fn mouse_button_down_event(&mut self, _ctx: &mut Context, _button: MouseButton, x: f32, y: f32);
  fn mouse_button_up_event(&mut self, _ctx: &mut Context, _button: MouseButton, _x: f32, _y: f32);
  fn mouse_motion_event(&mut self, _ctx: &mut Context, _x: f32, _y: f32, dx: f32, dy: f32);
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

  fn mouse_button_down_event(&mut self, ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
    for di in &mut self.draw_items {
      di.mouse_button_down_event(ctx, button, x, y);
    }
  }

  fn mouse_button_up_event(&mut self, ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
    for di in &mut self.draw_items {
      di.mouse_button_up_event(ctx, button, x, y);
    }
  }

  fn mouse_motion_event(&mut self, ctx: &mut Context, x: f32, y: f32, dx: f32, dy: f32) {
    for di in &mut self.draw_items {
      di.mouse_motion_event(ctx, x, y, dx, dy);
    }
  }
}
