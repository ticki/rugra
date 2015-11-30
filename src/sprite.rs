use sfml::graphics::{RenderWindow, RenderTarget, CircleShape, RectangleShape, Text, Color};
use sfml::graphics::Sprite as SfmlSprite;
use sfml::system::vector2::Vector2f;

use window::Window;
use texture::Texture;

pub struct Sprite {
    pub x: f32,
    pub y: f32,
    texture: Texture,
}

impl Sprite {
    pub fn new() -> Self {
        Sprite {
            x: 1.0,
            y: 1.0,
            texture: Texture::empty(),
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

    pub fn texture(&mut self, texture: Texture) -> &mut Self {
        self.texture = texture;
        self
    }

    pub fn draw(&self, window: &mut Window) {
        let mut shape = SfmlSprite::new().unwrap();
        shape.set_position2f(self.x, self.y);
        shape.set_texture(self.texture.to_sfml_texture(), true);

        window.sfml_window().draw(&mut shape);

    }
}
