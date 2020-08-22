use crate::image::*;
use crate::phase::*;
use ggez::*;

pub struct TitlePhase {
  draw_items: Vec<Box<dyn DrawItem>>,
  image_resources: ImageResources,
}

impl TitlePhase {
  pub fn new(ctx: &mut Context) -> GameResult<Self> {
    let image = DrawObject::new(300.0, 100.0);
    let draw_items: Vec<Box<dyn DrawItem>> = vec![Box::from(image)];

    Ok(Self {
      draw_items: draw_items,
      image_resources: ImageResources::new(ctx)?,
    })
  }
}

impl GamePhase for TitlePhase {
  fn update(&mut self, _ctx: &mut Context) -> PhaseAction {
    PhaseAction::Continue
  }

  fn draw(&self, ctx: &mut Context) -> GameResult {
    for di in &self.draw_items {
      di.draw(ctx, &self.image_resources)?;
    }

    Ok(())
  }
}
