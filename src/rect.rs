use sfml::graphics::{RenderWindow, RenderTarget, CircleShape, RectangleShape, Sprite, Text, Color};
use sfml::system::vector2::Vector2f;

use window::Window;

pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    color: [u8; 3],
    alpha: u8,
}

impl Rect {
    pub fn new(x: f32, y: f32) -> Self {
        Rect {
            x: x,
            y: y,
            w: 1.0,
            h: 1.0,
            color: [255; 3],
            alpha: 255,
        }
    }

    pub fn width(&mut self, w: f32) -> &mut Self {
        self.w = w;
        self
    }
    pub fn height(&mut self, h: f32) -> &mut Self {
        self.h = h;
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
        let mut shape = RectangleShape::new().unwrap();
        shape.set_position2f(self.x, self.y);
        shape.set_fill_color(&Color::new_rgba(self.color[0], self.color[1], self.color[2], self.alpha));
        shape.set_size(&Vector2f::new(self.w, self.h));

        window.sfml_window().draw(&mut shape);

    }
}
