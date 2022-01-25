use macroquad::prelude::*;

mod textures;
use textures::*;
mod player;
use player::*;
mod vecs;
use vecs::Vec2;
mod map;
use map::*;
mod camera;
use camera::Camera;

#[macroquad::main("GameJaaj7")]
async fn main() {
    let mut map = Map::from_file("map.txt", 15. * 6.);
    let textures = Textures::get().await;
    let mut player = Player::new(Vec2::new(10., 10.), &textures, 2., 4.);
    let mut camera = Camera {
        pos: Vec2::new(0., 0.),
        zoom: 1.,
    };

    let mut kind = 0;

    loop {
        clear_background(DARKGRAY);
        edit_map(&mut kind, &mut map, &mut camera);
        player.update(&mut camera);

        map.draw(&textures, &camera);
        player.draw(&camera);
        draw_icon(kind, &textures);
        next_frame().await
    }
}

fn edit_map(kind: &mut i8, map: &mut Map, camera: &mut Camera) {
    let x = (((mouse_position().0 - camera.pos.x) / camera.zoom) / map.size) as usize;
    let y = (((mouse_position().1 - camera.pos.y) / camera.zoom) / map.size) as usize;
    if x < map.width && y < map.height {
        if is_mouse_button_down(MouseButton::Left) {
            map.vec[y][x].kind = *kind as u8;
        } else if is_mouse_button_down(MouseButton::Right) {
            map.vec[y][x].kind = 0;
        }
    }

    if is_key_pressed(KeyCode::P) {
        map.to_file();
    }

    if is_key_down(KeyCode::LeftShift) {
        camera.zoom += mouse_wheel().1 / 10.;
    } else {
        *kind += mouse_wheel().1 as i8;
    }

    if *kind > 5 {
        *kind = 1;
    } else if *kind < 1 {
        *kind = 5;
    }
}

fn draw_icon(kind: i8, textures: &Textures) {
    let params = DrawTextureParams {
        dest_size: Some(macroquad::prelude::Vec2::new(30., 30.)),
        source: Some(Rect::new(kind as f32 * 15., 0., 15., 15.)),
        ..Default::default()
    };

    draw_texture_ex(textures.floors, 10., 10., WHITE, params);
}
