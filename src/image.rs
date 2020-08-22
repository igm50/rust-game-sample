use crate::state;
use ggez::*;

pub struct ImageResources {
  resources: Vec<graphics::Image>,
}

impl ImageResources {
  pub fn new(ctx: &mut Context) -> GameResult<Self> {
    let image_gu = graphics::Image::new(ctx, "/janken_gu.png")?;

    Ok(Self {
      resources: vec![image_gu],
    })
  }

  pub fn gu(&self) -> &graphics::Image {
    match self.resources.get(0) {
      Some(gu) => gu,
      None => panic!("画像が見つかりません"),
    }
  }
}

pub struct DrawObject {
  x: f32,
  y: f32,
  scale_x: f32,
  scale_y: f32,
}

impl DrawObject {
  pub fn new(x: f32, y: f32) -> Self {
    Self {
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

impl state::DrawItem for DrawObject {
  fn draw(&self, ctx: &mut Context, image_resources: &ImageResources) -> GameResult {
    graphics::draw(ctx, image_resources.gu(), self.draw_param())
  }
}
