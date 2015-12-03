use sfml::graphics::{RenderTarget, RectangleShape, Color};
use sfml::system::vector2::Vector2f;

use window::Window;

/// Rectangle
pub struct Rect<'a> {
    rect: RectangleShape<'a>,
}

impl<'a> Rect<'a> {
    /// Create new rectangle
    pub fn new() -> Self {
        Rect {
            rect: RectangleShape::new().unwrap(),
        }
    }

    /// Set the width of the rectangle
    pub fn width(&mut self, w: f32) -> &mut Self {
        let h = self.rect.get_size().y;
        self.rect.set_size(&Vector2f::new(w, h));
        self
    }

    /// Set the height of the rectangle
    pub fn height(&mut self, h: f32) -> &mut Self {
        let w = self.rect.get_size().x;
        self.rect.set_size(&Vector2f::new(w, h));
        self
    }

    /// Set the x coordinate of the rectangle
    pub fn x(&mut self, x: f32) -> &mut Self {
        let y = self.rect.get_position().y;
        self.rect.set_position(&Vector2f::new(x, y));
        self
    }

    /// Set the y coordinate of the rectangle
    pub fn y(&mut self, y: f32) -> &mut Self {
        let x = self.rect.get_position().x;
        self.rect.set_position(&Vector2f::new(x, y));
        self
    }

    /// Set the color of the rectangle
    pub fn color(&mut self, r: u8, g: u8, b: u8) -> &mut Self {
        self.rect.set_fill_color(&Color::new_rgb(r, g, b));
        self
    }

    /// Set the opacity of the rectangle
    pub fn alpha(&mut self, a: u8) -> &mut Self {
        let mut color = self.rect.get_fill_color();
        color.alpha = a;
        self.rect.set_fill_color(&color);
        self
    }

    /// Move the rectangle relative to the current position
    pub fn go(&mut self, x: f32, y: f32) -> &mut Self {
        let (px, py) = self.pos();
        self.set((px + x, py + y));
        self
    }

    /// Rotate the rectangle where a is given in degrees
    pub fn rotate(&mut self, a: f32) -> &mut Self {
        self.rect.rotate(a);
        self
    }

    /// Step in the current direction
    pub fn step(&mut self, size: f32) -> &mut Self {
        let angle = self.rect.get_rotation().to_radians();

        self.go(angle.cos() * size, angle.sin() * size);

        self
    }

    /// Get the length to a given point
    pub fn length_to(&self, (x, y): (f32, f32)) -> f32 {
        let pos = self.rect.get_position();

        let dx = x - pos.x;
        let dy = y - pos.y;

        (dx * dx + dy * dy).sqrt()
    }

    /// Get the position of the rectangle
    pub fn pos(&self) -> (f32, f32) {
        let pos = self.rect.get_position();

        (pos.x, pos.y)
    }

    /// Set the position of the rectangle
    pub fn set(&mut self, (x, y): (f32, f32)) -> &mut Self {
        self.x(x);
        self.y(y)
    }

    /// Draw the rectangle to a window
    pub fn draw(&mut self, window: &mut Window) {
        window.to_sfml_window().draw(&mut self.rect);

    }
}
