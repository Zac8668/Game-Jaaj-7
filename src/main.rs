use macroquad::prelude::*;

mod textures;
use textures::*;
mod player;
use player::*;
mod vecs;
use vecs::Vec2;
mod map;
use map::*;

#[macroquad::main("GameJaaj7")]
async fn main() {
    let mut map = Map::new(12, 12, 15. * 6.);
    let textures = Textures::get().await;
    let mut player = Player::new(Vec2::new(10., 10.), &textures, 2., 4.);

    let mut kind = 0;

    loop {
        clear_background(DARKGRAY);
        edit_map(&mut kind, &mut map);
        player.update();

        map.draw(&textures);
        player.draw();
        draw_icon(kind, &textures);
        next_frame().await
    }
}

fn edit_map(kind: &mut i8, map: &mut Map) {
    if is_mouse_button_pressed(MouseButton::Left) {
        let x = (mouse_position().0 / map.size) as usize;
        let y = (mouse_position().1 / map.size) as usize;

        map.vec[y][x].kind = *kind as u8;
    } else if is_mouse_button_pressed(MouseButton::Right) {
        let x = (mouse_position().0 / map.size) as usize;
        let y = (mouse_position().1 / map.size) as usize;

        map.vec[y][x].kind = 0;
    }
    *kind += mouse_wheel().1 as i8;

    if *kind > 5 {
        *kind = 0;
    } else if *kind < 0 {
        *kind = 5;
    }
}

fn draw_icon(kind: i8, textures: &Textures) {
    let params = DrawTextureParams {
        dest_size: Some(macroquad::prelude::Vec2::new(50., 50.)),
        ..Default::default()
    };

    let params = DrawTextureParams {
        dest_size: Some(macroquad::prelude::Vec2::new(30., 30.)),
        source: Some(Rect::new(kind as f32 * 15., 0., 15., 15.)),
        ..Default::default()
    };

    draw_texture_ex(
        textures.floors,
        10.,
        10.,
        WHITE,
        params.clone(),
    );
}
