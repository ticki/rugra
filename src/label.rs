use sfml::graphics::{RenderTarget, Text, TextStyle, Color};
use sfml::system::vector2::Vector2f;

use window::Window;
use font::Font;

/// A label (text)
pub struct Label<'a> {
    text: Text<'a>,
}

impl<'a> Label<'a> {
    /// Create a new label
    pub fn new(font: &'a Font) -> Self {
        let mut label = Label {
            text: Text::new().unwrap(),
        };

        label.text.set_font(font.to_sfml_font());

        label
    }

    /// Set the text
    pub fn text(&mut self, s: &str) -> &mut Self {
        self.text.set_string(s);
        self
    }

    /// Set the font size
    pub fn size(&mut self, s: u32) -> &mut Self {
        self.text.set_character_size(s);
        self
    }

    /// Set the x coordinate of the label
    pub fn x(&mut self, x: f32) -> &mut Self {
        let y = self.text.get_position().y;
        self.text.set_position(&Vector2f::new(x, y));
        self
    }

    /// Set the y coordinate of the label
    pub fn y(&mut self, y: f32) -> &mut Self {
        let x = self.text.get_position().x;
        self.text.set_position(&Vector2f::new(x, y));
        self
    }

    /// Set the color of the label
    pub fn color(&mut self, r: u8, g: u8, b: u8) -> &mut Self {
        self.text.set_color(&Color::new_rgb(r, g, b));
        self
    }

    /// Set the opacity of the label
    pub fn alpha(&mut self, a: u8) -> &mut Self {
        let mut color = self.text.get_color();
        color.alpha = a;
        self.text.set_color(&color);
        self
    }

    /// Make the label bold
    pub fn bold(&mut self) -> &mut Self {
        self.text.set_style(TextStyle::Bold);
        self
    }

    /// Make the label italic
    pub fn italic(&mut self) -> &mut Self {
        self.text.set_style(TextStyle::Italic);
        self
    }

    /// Make the label underlined
    pub fn underlined(&mut self) -> &mut Self {
        self.text.set_style(TextStyle::Underlined);
        self
    }

    /// Move the label relative to the current position
    pub fn go(&mut self, x: f32, y: f32) -> &mut Self {
        let (px, py) = self.pos();
        self.set((px + x, py + y));
        self
    }

    /// Rotate the label where a is given in degrees
    pub fn rotate(&mut self, a: f32) -> &mut Self {
        self.text.rotate(a);
        self
    }

    /// Step in the current direction
    pub fn step(&mut self, size: f32) -> &mut Self {
        let angle = self.text.get_rotation().to_radians();

        self.go(angle.cos() * size, angle.sin() * size);

        self
    }

    /// Get the length to a given point
    pub fn length_to(&self, (x, y): (f32, f32)) -> f32 {
        let pos = self.text.get_position();

        let dx = x - pos.x;
        let dy = y - pos.y;

        (dx * dx + dy * dy).sqrt()
    }

    /// Get the position of the label
    pub fn pos(&self) -> (f32, f32) {
        let pos = self.text.get_position();

        (pos.x, pos.y)
    }

    /// Set the position of the label
    pub fn set(&mut self, (x, y): (f32, f32)) -> &mut Self {
        self.x(x);
        self.y(y)
    }

    /// Draw the label on a window
    pub fn draw(&mut self, window: &mut Window) {
        window.to_sfml_window().draw(&mut self.text);
    }
}
