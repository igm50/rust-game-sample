use cgmath;
use ggez::event::EventHandler;
use ggez::graphics;
use ggez::input::mouse::MouseButton;
use ggez::*;

pub struct MainState {
  image1: graphics::Image,
}

impl MainState {
  pub fn new(ctx: &mut Context) -> GameResult<MainState> {
    let image1 = graphics::Image::new(ctx, "/sample.png")?;

    Ok(MainState { image1: image1 })
  }
}

impl EventHandler for MainState {
  fn update(&mut self, _ctx: &mut Context) -> GameResult {
    Ok(())
  }

  fn draw(&mut self, ctx: &mut Context) -> GameResult {
    graphics::clear(ctx, graphics::Color::new(1.0, 1.0, 1.0, 0.0));

    let dst = cgmath::Point2::new(300.0, 100.0);
    let params = graphics::DrawParam::new()
      .scale(mint::Vector2::from_slice(&[0.5, 0.5]))
      .dest(dst);
    graphics::draw(ctx, &self.image1, params)?;

    graphics::present(ctx)
  }

  fn mouse_button_down_event(&mut self, _ctx: &mut Context, _button: MouseButton, x: f32, y: f32) {
    println!("Pushed at x:{}, y:{}", x, y);
  }
}
