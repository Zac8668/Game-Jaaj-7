use macroquad::prelude::*;

mod textures;
use textures::*;
mod player;
use player::*;
mod vecs;
use vecs::Vec2;

#[macroquad::main("GameJaaj7")]
async fn main() {
    let textures = Textures::get().await;
    let mut player = Player::new(Vec2::new(10., 10.), &textures, 2., 10.);

    loop {
        clear_background(DARKGRAY);

        player.update();
        player.draw();

        next_frame().await
    }
}
