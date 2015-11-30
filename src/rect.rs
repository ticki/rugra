use sfml::graphics::{RenderTarget, RectangleShape, Color};
use sfml::system::vector2::Vector2f;

use window::Window;

pub struct Rect<'a> {
    rect: RectangleShape<'a>,
}

impl<'a> Rect<'a> {
    pub fn new() -> Self {
        Rect {
            rect: RectangleShape::new().unwrap(),
        }
    }

    pub fn width(&mut self, w: f32) -> &mut Self {
        let h = self.rect.get_size().y;
        self.rect.set_size(&Vector2f::new(w, h));
        self
    }

    pub fn height(&mut self, h: f32) -> &mut Self {
        let w = self.rect.get_size().x;
        self.rect.set_size(&Vector2f::new(w, h));
        self
    }

    pub fn x(&mut self, x: f32) -> &mut Self {
        let y = self.rect.get_position().y;
        self.rect.set_position(&Vector2f::new(x, y));
        self
    }

    pub fn y(&mut self, y: f32) -> &mut Self {
        let x = self.rect.get_position().x;
        self.rect.set_position(&Vector2f::new(x, y));
        self
    }

    pub fn color(&mut self, r: u8, g: u8, b: u8) -> &mut Self {
        self.rect.set_fill_color(&Color::new_rgb(r, g, b));
        self
    }

    pub fn alpha(&mut self, a: u8) -> &mut Self {
        let mut color = self.rect.get_fill_color();
        color.alpha = a;
        self.rect.set_fill_color(&color);
        self
    }

    pub fn go(&mut self, x: f32, y: f32) -> &mut Self {
        let (px, py) = self.pos();
        self.set((px + x, py + y));
        self
    }

    pub fn rotate(&mut self, a: f32) -> &mut Self {
        self.rect.rotate(a);
        self
    }

    pub fn step(&mut self, size: f32) -> &mut Self {
        let angle = size.to_radians();

        self.go(angle.cos() * size, angle.sin() * size);

        self
    }

    pub fn length_to(&self, (x, y): (f32, f32)) -> f32 {
        let pos = self.rect.get_position();

        let dx = x - pos.x;
        let dy = y - pos.y;

        (dx * dx + dy * dy).sqrt()
    }

    pub fn pos(&self) -> (f32, f32) {
        let pos = self.rect.get_position();

        (pos.x, pos.y)
    }

    pub fn set(&mut self, (x, y): (f32, f32)) -> &mut Self {
        self.x(x);
        self.y(y)
    }

    pub fn draw(&mut self, window: &mut Window) {
        window.sfml_window().draw(&mut self.rect);

    }
}
