use sfml::graphics::{RenderTarget, CircleShape, Color};
use sfml::system::vector2::Vector2f;

use window::Window;

pub struct Circle<'a> {
    circle: CircleShape<'a>,
}

impl<'a> Circle<'a> {
    pub fn new() -> Self {
        Circle {
            circle: CircleShape::new().unwrap(),
        }
    }

    pub fn radius(&mut self, r: f32) -> &mut Self {
        self.circle.set_radius(r);
        self
    }

    pub fn x(&mut self, x: f32) -> &mut Self {
        let (_, y) = self.pos();
        let r = self.circle.get_radius();
        self.circle.set_position(&Vector2f::new(x - r, y));
        self
    }

    pub fn y(&mut self, y: f32) -> &mut Self {
        let (x, _) = self.pos();
        let r = self.circle.get_radius();
        self.circle.set_position(&Vector2f::new(x, y - r));
        self
    }

    pub fn color(&mut self, r: u8, g: u8, b: u8) -> &mut Self {
        self.circle.set_fill_color(&Color::new_rgb(r, g, b));
        self
    }

    pub fn alpha(&mut self, a: u8) -> &mut Self {
        let mut color = self.circle.get_fill_color();
        color.alpha = a;
        self.circle.set_fill_color(&color);
        self
    }

    pub fn go(&mut self, x: f32, y: f32) -> &mut Self {
        let (px, py) = self.pos();
        self.set((px + x, py + y));
        self
    }

    pub fn rotate(&mut self, a: f32) -> &mut Self {
        self.circle.rotate(a);
        self
    }

    pub fn step(&mut self, size: f32) -> &mut Self {
        let angle = size.to_radians();

        self.go(angle.cos() * size, angle.sin() * size);

        self
    }

    pub fn length_to(&self, (x, y): (f32, f32)) -> f32 {
        let (px, py) = self.pos();

        let dx = x - px;
        let dy = y - py;

        (dx * dx + dy * dy).sqrt()
    }

    pub fn pos(&self) -> (f32, f32) {
        let pos = self.circle.get_position();

        let r = self.circle.get_radius();
        (pos.x + r, pos.y + r)
    }

    pub fn set(&mut self, (x, y): (f32, f32)) -> &mut Self {
        self.x(x);
        self.y(y)
    }

    pub fn draw(&mut self, window: &mut Window) {
        window.sfml_window().draw(&mut self.circle);

    }
}
