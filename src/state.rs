use crate::phase::*;
use ggez::event::EventHandler;
use ggez::input::keyboard::*;
use ggez::*;

pub struct MainState {
  phase: Box<dyn GamePhase>,
}

impl MainState {
  pub fn new(ctx: &mut Context) -> GameResult<Self> {
    Ok(Self {
      phase: Box::from(TitlePhase::new(ctx)?),
    })
  }
}

impl EventHandler for MainState {
  fn update(&mut self, ctx: &mut Context) -> GameResult {
    match self.phase.update(ctx) {
      PhaseAction::Continue => Ok(()),
      PhaseAction::Next => {
        self.phase = self.phase.next(ctx);
        Ok(())
      }
    }
  }

  fn draw(&mut self, ctx: &mut Context) -> GameResult {
    graphics::clear(ctx, graphics::Color::new(1.0, 1.0, 1.0, 0.0));

    self.phase.draw(ctx)?;

    graphics::present(ctx)
  }

  fn key_down_event(
    &mut self,
    ctx: &mut Context,
    keycode: KeyCode,
    _keymods: KeyMods,
    _repeat: bool,
  ) {
    self.phase.key_down_event(ctx, keycode);
  }
}
