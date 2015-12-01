use sfml::window::{ContextSettings, VideoMode, Close};
use sfml::window::event::Event;
use sfml::graphics::{RenderWindow, RenderTarget, Color, View};
use sfml::system::vector2::{Vector2f, ToVec};
use sfml::window::keyboard::Key as SfmlKey;

use key::{Key, SpecialKey};

use std::mem;
use std::thread;

/// The event state
///
/// Used for storing information of the events.
/// Is exposed through methods of `Window`.
pub struct EventState {
    pub right: bool,
    pub left: bool,
    pub chars: String,
    pub keys: Vec<SpecialKey>,
}

impl EventState {
    /// Create new default event state
    pub fn new() -> Self {
        EventState {
            right: false,
            left: false,
            chars: String::new(),
            keys: Vec::new(),
        }
    }
}

/// Window
pub struct Window {
    window: RenderWindow,
    /// The event state
    pub event: EventState,
}

impl Window {
    /// Create new window
    pub fn new(title: &str) -> Window {
        Window {
            window: RenderWindow::new(VideoMode::new_init(800, 600, 32),
                                      title,
                                      Close,
                                      &ContextSettings::default())
                        .expect("Could not construct window"),
            event: EventState::new(),
        }

    }

    /// Set width of the window
    pub fn width(&mut self, width: u32) -> &mut Window {
        let mut s = self.window.get_size();
        s.x = width;

        self.window.set_size(&s);
        self.goto(0.0, 0.0);

        self

    }

    /// Set the height of the window
    pub fn height(&mut self, height: u32) -> &mut Window {
        let mut s = self.window.get_size();
        s.y = height;

        self.window.set_size(&s);
        self.goto(0.0, 0.0);

        self

    }

    /// Get the mouse position
    pub fn mouse(&self) -> (f32, f32) {
        let pos = self.window.get_mouse_position().to_vector2f();

        (pos.x, pos.y)
    }

    /// Get the x coordinate of the mouse
    pub fn mx(&self) -> f32 {
        self.mouse().0
    }

    /// Get the y coordinate of the mouse
    pub fn my(&self) -> f32 {
        self.mouse().1
    }

    /// Check if the window is open
    pub fn is_open(&self) -> bool {
        self.window.is_open()
    }

    /// Is the left mouse button clicked?
    pub fn left_click(&self) -> bool {
        self.event.left
    }

    /// Is the right mouse button clicked?
    pub fn right_click(&self) -> bool {
        self.event.right
    }

    /// Is `k` (char or special key) pressed?
    pub fn key<T: Key>(&self, k: T) -> bool {
        k.is_pressed(self)
    }

    /// Poll the pressed chars
    pub fn poll_char(&mut self) -> String {
        mem::replace(&mut self.event.chars, String::new())
    }

    /// Clear the window
    pub fn clear(&mut self, r: u8, g: u8, b: u8) {
        self.window.clear(&Color::new_rgb(r, g, b));

        self.event.chars.clear();
        self.event.left = false;
        self.event.right = false;
    }

    /// Listen for events
    pub fn listen(&mut self) {

        for e in self.window.events() {
            match e {
                Event::TextEntered {
                    code: c
                } => self.event.chars.push(c),
                Event::MouseLeft => self.event.left = true,
                Event::MouseEntered => self.event.right = true,
                Event::Closed => self.window.close(),
                Event::Resized {
                    width,
                    height,
                } => {
                    self.width(width);
                    self.height(height);
                }
                Event::KeyPressed {
                    code: c,
                    ..
                } => {
                    match c {
                        SfmlKey::Escape => self.event.keys.push(SpecialKey::Escape),
                        SfmlKey::LControl | SfmlKey::RControl =>
                            self.event.keys.push(SpecialKey::Control),
                        SfmlKey::LShift | SfmlKey::RShift =>
                            self.event.keys.push(SpecialKey::Shift),
                        SfmlKey::LAlt | SfmlKey::RAlt => self.event.keys.push(SpecialKey::Alt),
                        SfmlKey::Return => self.event.keys.push(SpecialKey::Return),
                        SfmlKey::BackSpace => self.event.keys.push(SpecialKey::Backspace),
                        SfmlKey::Left => self.event.keys.push(SpecialKey::Left),
                        SfmlKey::Right => self.event.keys.push(SpecialKey::Right),
                        SfmlKey::Up => self.event.keys.push(SpecialKey::Up),
                        SfmlKey::Down => self.event.keys.push(SpecialKey::Down),
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }

    /// Update the window
    pub fn update(&mut self) -> bool {
        thread::sleep_ms(50);

        self.window.display();

        self.is_open()

    }

    /// Change view (camera) of the window
    pub fn goto(&mut self, x: f32, y: f32) {
        let size = self.window.get_size().to_vector2f();
        self.window.set_view(&View::new_init(&(Vector2f::new(x, y) + size / 2.0), &size).unwrap());
    }

    /// Convert to SFML (backend) window
    pub fn to_sfml_window(&mut self) -> &mut RenderWindow {
        &mut self.window
    }
}
