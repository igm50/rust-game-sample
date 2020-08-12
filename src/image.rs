use ggez::*;

pub struct Image {
  image: graphics::Image,
  x: f32,
  y: f32,
  scale_x: f32,
  scale_y: f32,
  clicked: bool,
}

impl Image {
  pub fn new(image: graphics::Image, x: f32, y: f32) -> Self {
    Self {
      image: image,
      x: x,
      y: y,
      scale_x: 0.5,
      scale_y: 0.5,
      clicked: false,
    }
  }

  fn width(&self) -> f32 {
    f32::from(self.image.width()) * self.scale_x
  }

  fn height(&self) -> f32 {
    f32::from(self.image.height()) * self.scale_y
  }

  fn x_is_in_area(&self, x: f32) -> bool {
    self.x < x && x < (self.x + self.width())
  }

  fn y_is_in_area(&self, y: f32) -> bool {
    self.y < y && y < (self.y + self.height())
  }

  fn is_in_area(&self, x: f32, y: f32) -> bool {
    self.x_is_in_area(x) && self.y_is_in_area(y)
  }

  pub fn click(&mut self, x: f32, y: f32) {
    if self.is_in_area(x, y) {
      self.clicked = true;
    }
  }

  pub fn un_click(&mut self) {
    self.clicked = false;
  }

  pub fn mouse_motion(&mut self, dx: f32, dy: f32) {
    if self.clicked {
      self.x += dx;
      self.y += dy;
    }
  }

  pub fn draw_param(&self) -> graphics::DrawParam {
    graphics::DrawParam::new()
      .scale(mint::Vector2::from_slice(&[self.scale_x, self.scale_y]))
      .dest(cgmath::Point2::new(self.x, self.y))
  }
}

impl graphics::Drawable for Image {
  fn draw(&self, ctx: &mut Context, param: graphics::DrawParam) -> GameResult {
    self.image.draw(ctx, param)
  }

  fn dimensions(&self, _ctx: &mut Context) -> Option<graphics::Rect> {
    Some(self.image.dimensions())
  }

  fn set_blend_mode(&mut self, mode: Option<graphics::BlendMode>) {
    self.image.set_blend_mode(mode);
  }

  fn blend_mode(&self) -> Option<graphics::BlendMode> {
    self.image.blend_mode()
  }
}
