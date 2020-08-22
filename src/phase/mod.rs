mod end;
mod title;
use ggez::input::keyboard::KeyCode;
use ggez::*;

pub enum PhaseAction {
  Continue,
  Next,
}

pub trait GamePhase {
  fn update(&mut self, ctx: &mut Context) -> &PhaseAction;
  fn draw(&self, ctx: &mut Context) -> GameResult;
  fn key_down_event(&mut self, ctx: &mut Context, keycode: KeyCode);
  fn next(&self, ctx: &mut Context) -> Box<dyn GamePhase>;
}

pub type TitlePhase = title::TitlePhase;
pub type EndPhase = end::EndPhase;
