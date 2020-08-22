use crate::phase::*;
use ggez::*;

pub struct TitlePhase {
  action: PhaseAction,
  title: graphics::Text,
  start: graphics::Text,
}

impl TitlePhase {
  pub fn new(_ctx: &mut Context) -> GameResult<Self> {
    let title_fragment = graphics::TextFragment::new("Title")
      .scale(graphics::Scale::uniform(150.0))
      .color(graphics::Color::from([0.0, 0.0, 0.0, 1.0]));
    let title = graphics::Text::new(title_fragment);

    let start_fragment = graphics::TextFragment::new("Start")
      .scale(graphics::Scale::uniform(150.0))
      .color(graphics::Color::from([0.0, 0.0, 0.0, 1.0]));
    let start = graphics::Text::new(start_fragment);

    Ok(Self {
      action: PhaseAction::Continue,
      title: title,
      start: start,
    })
  }
}

impl GamePhase for TitlePhase {
  fn update(&mut self, _ctx: &mut Context) -> &PhaseAction {
    &self.action
  }

  fn draw(&self, ctx: &mut Context) -> GameResult {
    let param = graphics::DrawParam::new().dest(cgmath::Point2::new(300.0, 100.0));
    graphics::draw(ctx, &self.title, param)?;

    let param = graphics::DrawParam::new().dest(cgmath::Point2::new(300.0, 300.0));
    graphics::draw(ctx, &self.start, param)?;

    Ok(())
  }

  fn key_down_event(&mut self, _ctx: &mut Context, keycode: KeyCode) {
    if let KeyCode::Return = keycode {
      self.action = PhaseAction::Next;
    }
  }

  fn next(&self, _ctx: &mut Context) -> Box<dyn GamePhase> {
    Box::from(EndPhase::new())
  }
}
