use macroquad::prelude::*;

mod textures;
use textures::*;
mod player;
use player::*;
mod vecs;
use vecs::Vec2;

#[macroquad::main("BasicShapes")]
async fn main() {
    let textures = Textures::get().await;
    let mut player = Player::new(Vec2::new(10., 10.), &textures, 2.);

    loop {
        player.update();
        player.draw();

        next_frame().await
    }
}
