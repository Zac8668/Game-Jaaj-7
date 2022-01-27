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
    //let mut floors = Map::from_file("assets/floors.txt", 15. * 6., false);
    //let mut walls = Map::from_file("assets/walls.txt", 15. * 6., true);
    let mut floors = Map::new(10, 10, 15. * 6., false);
    let mut walls = Map::new(10, 10, 15. * 6., true);
    let textures = Textures::get().await;

    let mut player = Player::new(Vec2::new(900., 900.), &textures, 2., 4.);
    let mut camera = Camera {
        pos: Vec2::new(0., 0.),
        zoom: 1.,
    };

    let mut kind = 0;
    let mut wall = false;

    loop {
        clear_background(DARKGRAY);
        edit_map(
            &mut kind,
            &mut walls,
            &mut floors,
            &mut camera,
            &textures,
            &mut wall,
        );
        player.update(&mut camera, &walls);

        floors.draw(&textures, &camera);
        walls.draw(&textures, &camera);
        player.draw(&textures, &camera, &walls);
        draw_icon(kind, &textures, &wall);
        next_frame().await
    }
}

fn edit_map(
    kind: &mut i8,
    walls: &mut Map,
    floors: &mut Map,
    camera: &mut Camera,
    textures: &Textures,
    wall: &mut bool,
) {
    let x = (((mouse_position().0 - camera.pos.x) / camera.zoom) / walls.size) as usize;
    let y = (((mouse_position().1 - camera.pos.y) / camera.zoom) / walls.size) as usize;
    let map = if *wall { walls } else { floors };

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
    if is_key_pressed(KeyCode::F) {
        *wall = !*wall;
    }

    let scroll = if mouse_wheel().1 > 0. {
        1.
    } else if mouse_wheel().1 < 0. {
        -1.
    } else {
        0.
    };
    if is_key_down(KeyCode::LeftShift) {
        camera.zoom += scroll / 10.;
    } else {
        *kind += scroll as i8;
    }

    let n = if *wall {
        (textures.walls.width() / (map.size / 6.)) as i8
    } else {
        (textures.floors.width() / (map.size / 6.)) as i8
    };

    if *kind > n - 1 {
        *kind = 1;
    } else if *kind < 1 {
        *kind = n - 1;
    }
}

fn draw_icon(kind: i8, textures: &Textures, wall: &bool) {
    if *wall {
        let params = DrawTextureParams {
            dest_size: Some(macroquad::prelude::Vec2::new(30., 48.)),
            source: Some(Rect::new(kind as f32 * 15., 0., 15., 24.)),
            ..Default::default()
        };
        draw_texture_ex(textures.walls, 10., 10., WHITE, params);
    } else {
        let params = DrawTextureParams {
            dest_size: Some(macroquad::prelude::Vec2::new(30., 30.)),
            source: Some(Rect::new(kind as f32 * 15., 0., 15., 15.)),
            ..Default::default()
        };
        draw_texture_ex(textures.floors, 10., 10., WHITE, params);
    }
}
