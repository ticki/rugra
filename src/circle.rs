use sfml::graphics::{RenderWindow, RenderTarget, CircleShape, RectangleShape, Sprite, Text, Color};
use sfml::system::vector2::Vector2f;

use window::Window;

pub struct Circle {
    pub x: f32,
    pub y: f32,
    pub radius: f32,
    color: [u8; 3],
    alpha: u8,
}

impl Circle {
    pub fn new(x: f32, y: f32) -> Self {
        Circle {
            x: x,
            y: y,
            radius: 1.0,
            color: [255; 3],
            alpha: 255,
        }
    }

    pub fn radius(&mut self, r: f32) -> &mut Self {
        self.radius = r;
        self
    }

    pub fn x(&mut self, x: f32) -> &mut Self {
        self.x = x;
        self
    }

    pub fn y(&mut self, y: f32) -> &mut Self {
        self.y = y;
        self
    }

    pub fn color(&mut self, r: u8, g: u8, b: u8) -> &mut Self {
        self.color = [r, g, b];
        self
    }

    pub fn alpha(&mut self, a: u8) -> &mut Self {
        self.alpha = a;
        self
    }

    pub fn draw(&self, window: &mut Window) {
        let mut shape = CircleShape::new().unwrap();
        shape.set_position2f(self.x, self.y);
        shape.set_fill_color(&Color::new_rgba(self.color[0], self.color[1], self.color[2], self.alpha));
        shape.set_radius(self.radius);

        window.sfml_window().draw(&mut shape);

    }
}
