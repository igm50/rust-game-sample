use crate::phase::*;
use ggez::*;

pub struct EndPhase {}

impl EndPhase {
  pub fn new() -> Self {
    Self {}
  }
}

impl GamePhase for EndPhase {
  fn update(&mut self, ctx: &mut Context) -> &PhaseAction {
    event::quit(ctx);
    &PhaseAction::Continue
  }

  fn draw(&self, _ctx: &mut Context) -> GameResult {
    Ok(())
  }

  fn key_down_event(&mut self, _ctx: &mut Context, _keycode: KeyCode) {}

  fn next(&self, _ctx: &mut Context) -> Box<dyn GamePhase> {
    Box::from(Self::new())
  }
}
