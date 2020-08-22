mod title;
use ggez::*;

pub enum PhaseAction {
  Continue,
}

pub trait GamePhase {
  fn update(&mut self, ctx: &mut Context) -> PhaseAction;
  fn draw(&self, ctx: &mut Context) -> GameResult;
}

pub type TitlePhase = title::TitlePhase;
