use crate::image;
use ggez::event::EventHandler;
use ggez::*;
use std::boxed::Box;

// phase
pub enum PhaseAction {
  Continue,
}

pub trait GamePhase {
  fn update(&mut self, ctx: &mut Context) -> PhaseAction;
  fn draw(&self, ctx: &mut Context) -> GameResult;
}

struct TitlePhase {
  draw_items: Vec<Box<dyn DrawItem>>,
  image_resources: image::ImageResources,
}

impl TitlePhase {
  fn new(ctx: &mut Context) -> GameResult<Self> {
    let image = image::DrawObject::new(300.0, 100.0);
    let draw_items: Vec<Box<dyn DrawItem>> = vec![Box::from(image)];

    Ok(Self {
      draw_items: draw_items,
      image_resources: image::ImageResources::new(ctx)?,
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

pub trait DrawItem {
  fn draw(&self, ctx: &mut Context, image_resources: &image::ImageResources) -> GameResult;
}

// main state
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
    }
  }

  fn draw(&mut self, ctx: &mut Context) -> GameResult {
    graphics::clear(ctx, graphics::Color::new(1.0, 1.0, 1.0, 0.0));

    self.phase.draw(ctx)?;

    graphics::present(ctx)
  }
}
