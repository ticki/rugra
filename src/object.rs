use sfml::traits::Drawable;
use sfml::graphics::{RenderWindow, RenderTarget, CircleShape, RectangleShape, Sprite, Text};

pub enum Object<'a> {
    Circle(CircleShape<'a>),
    Sprite(Sprite<'a>),
    Rect(RectangleShape<'a>),
    Text(Text<'a>),
}

impl<'a> Object<'a> {
    pub fn draw(&mut self, window: &mut RenderWindow) {
            //self.window.draw(&d.drawable);
        match *self {
            Object::Circle(ref mut x) => window.draw(x),
            Object::Sprite(ref mut x) => window.draw(x),
            Object::Rect(ref mut x) => window.draw(x),
            Object::Text(ref mut x) => window.draw(x),
        }
    }
}
