use sfml::window::{ContextSettings, VideoMode, Close};
use sfml::window::event::Event;
use sfml::graphics::{RenderWindow, RenderTarget, Color, View};
use sfml::system::vector2::Vector2f;

use std::mem;
use std::thread;

pub struct EventState {
    right: bool,
    left: bool,
    keys: String,
}

impl EventState {
    pub fn new() -> Self {
        EventState {
            right: false,
            left: false,
            keys: String::new(),
        }
    }
}

pub struct Window {
    window: RenderWindow,
    event: EventState,
}

impl Window {
    pub fn new(title: &str) -> Window {
        Window {
            window: RenderWindow::new(VideoMode::new_init(800, 600, 32),
                                      title,
                                      Close,
                                      &ContextSettings::default()).expect("Could not construct window"),
            event: EventState::new(),
        }

    }

    pub fn width(&mut self, width: u32) -> &mut Window {
        let mut s = self.window.get_size();
        s.x = width;

        self.window.set_size(&s);
        self.goto(0.0, 0.0);

        self

    }

    pub fn height(&mut self, height: u32) -> &mut Window {
        let mut s = self.window.get_size();
        s.y = height;

        self.window.set_size(&s);
        self.goto(0.0, 0.0);

        self

    }

    pub fn mouse(&self) -> (u32, u32) {
        let pos = self.window.get_mouse_position();

        (pos.x as u32, pos.y as u32)
    }

    pub fn mx(&self) -> u32 {
        self.mouse().0
    }

    pub fn my(&self) -> u32 {
        self.mouse().1
    }

    pub fn is_open(&self) -> bool {
        self.window.is_open()
    }

    pub fn left_click(&self) -> bool {
        self.event.left
    }

    pub fn right_click(&self) -> bool {
        self.event.right
    }

    pub fn is_pressed(&self, c: char) -> bool {
        self.event.keys.contains(c)
    }

    pub fn poll_char(&mut self) -> String {
        mem::replace(&mut self.event.keys, String::new())
    }

    pub fn clear(&mut self, r: u8, g: u8, b: u8) {
        self.window.clear(&Color::new_rgb(r, g, b));

        self.event.keys.clear();
        self.event.left = false;
        self.event.right = false;
    }

    pub fn listen(&mut self) {

        for e in self.window.events() {
            match e {
                Event::TextEntered {
                    code: c
                } => self.event.keys.push(c),
                Event::MouseLeft => self.event.left = true,
                Event::MouseEntered => self.event.right = true,
                _ => {},
            }
        }
    }

    pub fn update(&mut self) -> bool {
        thread::sleep_ms(50);

        self.window.display();

        self.is_open()

    }

    pub fn goto(&mut self, x: f32, y: f32) {
        let size = self.window.get_size();
        self.window.set_view(&View::new_init(&Vector2f::new(x, y), &Vector2f::new(size.x as f32, size.y as f32)).unwrap());
    }

    pub fn sfml_window(&mut self) -> &mut RenderWindow {
        &mut self.window
    }
}
