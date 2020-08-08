use ggez::event::EventHandler;
use ggez::input::mouse::MouseButton;
use ggez::*;
use std::time::Duration;

pub struct GameState {
  dt: Duration,
}

impl GameState {
  pub fn new(dt: Duration) -> Self {
    Self { dt: dt }
  }
}

impl EventHandler for GameState {
  fn update(&mut self, ctx: &mut Context) -> GameResult {
    self.dt = timer::delta(ctx);
    Ok(())
  }

  fn draw(&mut self, _ctx: &mut Context) -> GameResult {
    Ok(())
  }

  fn mouse_button_down_event(&mut self, _ctx: &mut Context, _button: MouseButton, x: f32, y: f32) {
    println!("Pushed at x:{}, y:{}", x, y);
  }
}
