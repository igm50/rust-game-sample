use crate::state;
use ggez::*;

pub struct Image {
  image: graphics::Image,
  x: f32,
  y: f32,
  scale_x: f32,
  scale_y: f32,
}

impl Image {
  pub fn new(image: graphics::Image, x: f32, y: f32) -> Self {
    Self {
      image: image,
      x: x,
      y: y,
      scale_x: 0.5,
      scale_y: 0.5,
    }
  }

  fn draw_param(&self) -> graphics::DrawParam {
    graphics::DrawParam::new()
      .scale(mint::Vector2::from_slice(&[self.scale_x, self.scale_y]))
      .dest(cgmath::Point2::new(self.x, self.y))
  }
}

impl state::DrawItem for Image {
  fn draw(&self, ctx: &mut Context) -> GameResult {
    graphics::draw(ctx, &self.image, self.draw_param())
  }
}
