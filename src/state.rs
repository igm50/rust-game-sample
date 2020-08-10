use cgmath;
use ggez::event::EventHandler;
use ggez::graphics;
use ggez::input::mouse::MouseButton;
use ggez::*;

pub struct MainState {
  image1: graphics::Image,
  x: f32,
  y: f32,
  mouse_down: bool,
}

impl MainState {
  pub fn new(ctx: &mut Context) -> GameResult<MainState> {
    let image1 = graphics::Image::new(ctx, "/sample.png")?;

    Ok(MainState {
      image1: image1,
      x: 300.0,
      y: 100.0,
      mouse_down: false,
    })
  }
}

impl EventHandler for MainState {
  fn update(&mut self, _ctx: &mut Context) -> GameResult {
    Ok(())
  }

  fn draw(&mut self, ctx: &mut Context) -> GameResult {
    graphics::clear(ctx, graphics::Color::new(1.0, 1.0, 1.0, 0.0));

    let dst = cgmath::Point2::new(self.x, self.y);
    let params = graphics::DrawParam::new()
      .scale(mint::Vector2::from_slice(&[0.5, 0.5]))
      .dest(dst);
    graphics::draw(ctx, &self.image1, params)?;

    graphics::present(ctx)
  }

  fn mouse_button_down_event(
    &mut self,
    _ctx: &mut Context,
    _button: MouseButton,
    _x: f32,
    _y: f32,
  ) {
    self.mouse_down = true;
  }

  fn mouse_button_up_event(&mut self, _ctx: &mut Context, _button: MouseButton, _x: f32, _y: f32) {
    self.mouse_down = false;
  }

  fn mouse_motion_event(&mut self, _ctx: &mut Context, _x: f32, _y: f32, dx: f32, dy: f32) {
    if self.mouse_down {
      self.x = self.x + dx;
      self.y = self.y + dy;
    }
  }
}
