extern crate rugra;

use rugra::prelude::*;

fn main() {
    let mut window = Window::new("Hey there");
    window.width(500).height(500);
    let mut x = 10.0;

    let font = Font::load("assets/font.ttf");
    let icon = Texture::load("assets/icon.png");

    let mut player = Sprite::new();
    player
         .texture(&icon)
         .x(0.1).y(x * x);

    while window.update() {
        window.clear(255, 255, 255);
        window.listen();

        if window.key('d') {
            player.rotate(10.0);
        }
        if window.key('a') {
            player.rotate(-10.0);
        }
        if window.key(' ') {
            player.step(10.0);
        }



        if window.key('q') || window.key(Escape) {
            break;
        }

        player.draw(&mut window);


    }


}
