use window::Window;

pub trait Key {
    fn is_pressed(&self, window: &Window) -> bool;
}

#[derive(PartialEq, Clone, Copy)]
pub enum SpecialKey {
    Escape,
    Control,
    Shift,
    Alt,
    Return,
    Backspace,
    Left,
    Right,
    Up,
    Down,
}

impl Key for char {
    fn is_pressed(&self, window: &Window) -> bool {
        window.event.chars.contains(|ref c| c == self)
    }
}

impl Key for SpecialKey {
    fn is_pressed(&self, window: &Window) -> bool {
        window.event.keys.contains(self)
    }
}
