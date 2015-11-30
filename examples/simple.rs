extern crate rugra;

use rugra::prelude::*;

fn main() {
    let mut window = Window::new("Hey there");
    window.width(500).height(500);
    let mut x = 10.0;

    while window.update() {
        window.clear(0, 0, 0);
        window.listen();

        if window.is_pressed('d') {
            x += 2.0;
        }
        if window.is_pressed('a') {
            x -= 2.0;
        }

        if window.is_pressed('q') {
            break;
        }

        Rect::new(x, 10.0).width(100.0).height(50.0).color(50, 50, 0).draw(&mut window);
    }


}
