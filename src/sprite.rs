use sfml::graphics::RenderTarget;
use sfml::graphics::Sprite as SfmlSprite;

use window::Window;
use texture::Texture;

pub struct Sprite<'a> {
    sprite: SfmlSprite<'a>,
}

impl<'a> Sprite<'a> {
    pub fn new() -> Self {
        Sprite {
            sprite: SfmlSprite::new().unwrap(),
        }
    }

    pub fn x(&mut self, x: f32) -> &mut Self {
        let y = self.sprite.get_position().y;
        self.sprite.set_position2f(x, y);
        self
    }

    pub fn y(&mut self, y: f32) -> &mut Self {
        let x = self.sprite.get_position().x;
        self.sprite.set_position2f(x, y);
        self
    }

    pub fn texture(&mut self, texture: &'a Texture) -> &mut Self {
        self.sprite.set_texture(texture.to_sfml_texture(), true);
        self
    }

    pub fn sfml_sprite(&mut self) -> &mut SfmlSprite<'a> {
        &mut self.sprite
    }

    pub fn draw(&mut self, window: &mut Window) {
        window.sfml_window().draw(&mut self.sprite);
    }

    pub fn go(&mut self, x: f32, y: f32) -> &mut Self {
        let (px, py) = self.pos();
        self.set((px + x, py + y));
        self
    }

    pub fn rotate(&mut self, a: f32) -> &mut Self {
        self.sprite.rotate(a);
        self
    }

    pub fn step(&mut self, size: f32) -> &mut Self {
        let angle = size.to_radians();

        self.go(angle.cos() * size, angle.sin() * size);

        self
    }

    pub fn length_to(&self, (x, y): (f32, f32)) -> f32 {
        let pos = self.sprite.get_position();

        let dx = x - pos.x;
        let dy = y - pos.y;

        (dx * dx + dy * dy).sqrt()
    }

    pub fn set(&mut self, (x, y): (f32, f32)) -> &mut Self {
        self.x(x);
        self.y(y)
    }

    pub fn pos(&self) -> (f32, f32) {
        let pos = self.sprite.get_position();

        (pos.x, pos.y)
    }
}
