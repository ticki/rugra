extern crate rugra;

use rugra::prelude::*;

fn main() {
    let mut window = Window::new("Hey there");
    window.width(500).height(500);
    let mut x = 10.0;

    while window.update() {
        window.clear(0, 0, 0);
        window.listen();

        if window.key('d') {
            x += 2.0;
        }
        if window.key('a') {
            x -= 2.0;
        }
        if window.key('q') || window.key(Escape) {
            break;
        }

        let mut circle = Circle::new();

        circle
            .x(x).y(10.0)
            .radius(100.0)
            .color(50, 250, 0);

        if circle.length_to(window.mouse()) < 50.0 {
            circle.color(0, 0, 255);
        }

        circle.draw(&mut window);

        let font = Font::load("assets/font.ttf");
        let icon = Texture::load("assets/icon.png");

        Label::new(&font)
            .text("Press A or D")
            .x(0.0).y(x)
            .color(255, 255, 255)
            .size(30)
            .draw(&mut window);

        Circle::new()
            .set(window.mouse())
            .color(255, 255, 255)
            .radius(30.0)
            .draw(&mut window);

        Sprite::new()
            .texture(&icon)
            .x(0.1).y(x * x)
            .draw(&mut window);
    }


}
