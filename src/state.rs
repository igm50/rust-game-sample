use crate::image::Image;
use ggez::event::EventHandler;
use ggez::input::mouse::MouseButton;
use ggez::*;

pub struct MainState {
  image: Image,
}

impl MainState {
  pub fn new(ctx: &mut Context) -> GameResult<MainState> {
    let image = Image::new(graphics::Image::new(ctx, "/sample.png")?, 300.0, 100.0);

    Ok(MainState { image: image })
  }

  fn draw_params(&self) -> graphics::DrawParam {
    self.image.draw_param()
  }
}

impl EventHandler for MainState {
  fn update(&mut self, _ctx: &mut Context) -> GameResult {
    Ok(())
  }

  fn draw(&mut self, ctx: &mut Context) -> GameResult {
    graphics::clear(ctx, graphics::Color::new(1.0, 1.0, 1.0, 0.0));

    graphics::draw(ctx, &self.image, self.draw_params())?;

    graphics::present(ctx)
  }

  fn mouse_button_down_event(&mut self, _ctx: &mut Context, _button: MouseButton, x: f32, y: f32) {
    self.image.click(x, y);
  }

  fn mouse_button_up_event(&mut self, _ctx: &mut Context, _button: MouseButton, _x: f32, _y: f32) {
    self.image.un_click();
  }

  fn mouse_motion_event(&mut self, _ctx: &mut Context, _x: f32, _y: f32, dx: f32, dy: f32) {
    if self.image.clicked {
      self.image.add(dx, dy);
    }
  }
}
