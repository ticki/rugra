use sfml::graphics::{RenderTarget, CircleShape, Color};
use sfml::system::vector2::Vector2f;

use window::Window;

/// An circle
pub struct Circle<'a> {
    circle: CircleShape<'a>,
}

impl<'a> Circle<'a> {
    /// Create a new circle
    pub fn new() -> Self {
        Circle {
            circle: CircleShape::new().unwrap(),
        }
    }

    /// Set the radius of the circle
    pub fn radius(&mut self, r: f32) -> &mut Self {
        self.circle.set_radius(r);
        self
    }

    /// Set the x value of the circle
    pub fn x(&mut self, x: f32) -> &mut Self {
        let (_, y) = self.pos();
        let r = self.circle.get_radius();
        self.circle.set_position(&Vector2f::new(x - r, y));
        self
    }

    /// Set the y value of the circle
    pub fn y(&mut self, y: f32) -> &mut Self {
        let (x, _) = self.pos();
        let r = self.circle.get_radius();
        self.circle.set_position(&Vector2f::new(x, y - r));
        self
    }

    /// Set the color of the circle
    pub fn color(&mut self, r: u8, g: u8, b: u8) -> &mut Self {
        self.circle.set_fill_color(&Color::new_rgb(r, g, b));
        self
    }

    /// Set the opacity of the circle
    pub fn alpha(&mut self, a: u8) -> &mut Self {
        let mut color = self.circle.get_fill_color();
        color.alpha = a;
        self.circle.set_fill_color(&color);
        self
    }

    /// Move relative to the current position
    pub fn go(&mut self, x: f32, y: f32) -> &mut Self {
        let (px, py) = self.pos();
        self.set((px + x, py + y));
        self
    }

    /// Rotate where a is given in degrees
    pub fn rotate(&mut self, a: f32) -> &mut Self {
        self.circle.rotate(a);
        self
    }

    /// Step in the current direction
    pub fn step(&mut self, size: f32) -> &mut Self {
        let angle = self.circle.get_rotation().to_radians();

        self.go(angle.cos() * size, angle.sin() * size);

        self
    }

    /// Get the length to a given point
    pub fn length_to(&self, (x, y): (f32, f32)) -> f32 {
        let (px, py) = self.pos();

        let dx = x - px;
        let dy = y - py;

        (dx * dx + dy * dy).sqrt()
    }

    /// Get the position of the circle
    pub fn pos(&self) -> (f32, f32) {
        let pos = self.circle.get_position();

        let r = self.circle.get_radius();
        (pos.x + r, pos.y + r)
    }

    /// Set the position of the circle
    pub fn set(&mut self, (x, y): (f32, f32)) -> &mut Self {
        self.x(x);
        self.y(y)
    }

    /// Draw the circle on a window
    pub fn draw(&mut self, window: &mut Window) {
        window.to_sfml_window().draw(&mut self.circle);

    }
}
