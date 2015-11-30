use sfml::graphics::{RenderWindow, RenderTarget, CircleShape, RectangleShape, Text, Color, Texture};
use sfml::graphics::Sprite as SfmlSprite;
use sfml::system::vector2::Vector2f;

use window::Window;

pub struct Sprite {
    pub x: f32,
    pub y: f32,
    texture: Texture,
}

impl Sprite {
    pub fn new(f: &str) -> Self {
        Sprite {
            x: 1.0,
            y: 1.0,
            w: 1.0,
            h: 1.0,
            color: [255; 3],
            alpha: 255,
            texture: Texture::new_from_file(f).expect("Couldn't load sprite"),
        }
    }

    pub fn x(&mut self, x: f32) -> &mut Self {
        self.x = x;
        self
    }

    pub fn y(&mut self, y: f32) -> &mut Self {
        self.y = y;
        self
    }

    pub fn draw(&self, window: &mut Window) {
        let mut shape = SfmlSprite::new().unwrap();
        shape.set_position2f(self.x, self.y);
        shape.set_texture(&self.texture, true);

        window.sfml_window().draw(&mut shape);

    }
}
