use sfml::graphics::{RenderWindow, RenderTarget, CircleShape, Text, TextStyle, Color};
use sfml::system::vector2::Vector2f;

use window::Window;
use font::Font;

pub struct Label<'a> {
    text: Text<'a>,
}

impl<'a> Label<'a> {
    pub fn new(font: &'a Font) -> Self {
        let mut label = Label {
            text: Text::new().unwrap(),
        };

        label.text.set_font(font.to_sfml_font());

        label
    }

    pub fn text(&mut self, s: &str) -> &mut Self {
        self.text.set_string(s);
        self
    }

    pub fn size(&mut self, s: u32) -> &mut Self {
        self.text.set_character_size(s);
        self
    }

    pub fn x(&mut self, x: f32) -> &mut Self {
        let y = self.text.get_position().y;
        self.text.set_position(&Vector2f::new(x, y));
        self
    }

    pub fn y(&mut self, y: f32) -> &mut Self {
        let x = self.text.get_position().x;
        self.text.set_position(&Vector2f::new(x, y));
        self
    }

    pub fn color(&mut self, r: u8, g: u8, b: u8) -> &mut Self {
        self.text.set_color(&Color::new_rgb(r, g, b));
        self
    }

    pub fn alpha(&mut self, a: u8) -> &mut Self {
        let mut color = self.text.get_color();
        color.alpha = a;
        self.text.set_color(&color);
        self
    }

    pub fn bold(&mut self) -> &mut Self {
        self.text.set_style(TextStyle::Bold);
        self
    }

    pub fn italic(&mut self) -> &mut Self {
        self.text.set_style(TextStyle::Italic);
        self
    }

    pub fn underlined(&mut self) -> &mut Self {
        self.text.set_style(TextStyle::Underlined);
        self
    }

    pub fn go(&mut self, x: f32, y: f32) -> &mut Self {
        let pos = self.text.get_position();
        self.text.set_position(&(pos + Vector2f::new(x, y)));
        self
    }

    pub fn rotate(&mut self, a: f32) -> &mut Self {
        self.text.rotate(a);
        self
    }

    pub fn step(&mut self, size: f32) -> &mut Self {
        let angle = size.to_radians();

        self.go(angle.cos() * size, angle.sin() * size);

        self
    }

    pub fn draw(&mut self, window: &mut Window) {
        window.sfml_window().draw(&mut self.text);

    }
}
