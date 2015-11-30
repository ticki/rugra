use sfml::graphics::{RenderWindow, RenderTarget, CircleShape, Text, Color};
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
        let y = self.circle.get_position().y;
        self.circle.set_position(&Vector2f::new(x, y));
        self
    }

    pub fn y(&mut self, y: f32) -> &mut Self {
        let x = self.circle.get_position().x;
        self.circle.set_position(&Vector2f::new(x, y));
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

    pub fn draw(&mut self, window: &mut Window) {
        window.sfml_window().draw(&mut self.circle);

    }
}
