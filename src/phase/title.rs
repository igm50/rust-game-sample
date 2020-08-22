use crate::phase::*;
use ggez::*;

pub struct TitlePhase {
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
      title: title,
      start: start,
    })
  }
}

impl GamePhase for TitlePhase {
  fn update(&mut self, _ctx: &mut Context) -> PhaseAction {
    PhaseAction::Continue
  }

  fn draw(&self, ctx: &mut Context) -> GameResult {
    let param = graphics::DrawParam::new().dest(cgmath::Point2::new(300.0, 100.0));
    graphics::draw(ctx, &self.title, param)?;

    let param = graphics::DrawParam::new().dest(cgmath::Point2::new(300.0, 300.0));
    graphics::draw(ctx, &self.start, param)?;

    Ok(())
  }
}
