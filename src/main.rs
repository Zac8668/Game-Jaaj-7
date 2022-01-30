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
mod animation;
mod enemies;
use enemies::Buffalo;
mod history;
use history::*;

#[macroquad::main("GameJaaj7")]
async fn main() {
    //add loading screen here
    let textures = Textures::get().await;
    let mut floors =
        Map::from_file("assets/world-data/floors.txt", 15. * 6., false, &textures).await;
    let mut walls = Map::from_file("assets/world-data/walls.txt", 15. * 6., true, &textures).await;
    let mut player = Player::new(
        Vec2::new(
            14. * walls.size + walls.size / 2.,
            12. * walls.size + walls.size / 2.,
        ),
        &textures,
        2.,
        8.,
    );
    let real_size = vec![
        player.sprite.animations[player.sprite.cur_animation].width as f32,
        player.size * player.sprite.animations[player.sprite.cur_animation].height as f32,
    ];
    player.pos.x -= real_size[0];
    player.pos.y -= real_size[1] / 2.;
    let mut buff = Buffalo::new(Vec2::new(0., 0.), &textures, 2., 8.);
    let mut camera = Camera {
        pos: Vec2::new(90., 90.),
        zoom: 1.,
        speed: Vec2::new(0., 0.),
        speed_limit: Vec2::new(50., 20.),
    };
    let mut kind = 1;
    let mut wall = false;
    let mut scene = 0;
    let mut exit = false;

    let mut history = History::new(&textures);

    loop {
        match scene {
            0 => history.tick(&mut scene),
            1 => menu(&textures, &mut scene, &mut exit),
            2 => in_game(
                &mut kind,
                &mut walls,
                &mut floors,
                &mut camera,
                &textures,
                &mut wall,
                &mut player,
                &mut buff,
            ),
            _ => (),
        };

        if exit {
            break;
        }
        next_frame().await
    }
}

fn menu(textures: &Textures, scene: &mut i32, exit: &mut bool) {
    draw_texture(textures.menu_art, 0., 0., WHITE);

    let new_pos = Vec2::new(500., 200.);
    let new_x = mouse_position().0 - new_pos.x;
    let new_y = mouse_position().1 - new_pos.y;
    let new_in = new_x < textures.new_game.width()
        && new_x > 0.
        && new_y > 0.
        && new_y < textures.new_game.height();
    draw_texture(
        textures.new_game,
        new_pos.x,
        new_pos.y,
        if new_in { LIGHTGRAY } else { WHITE },
    );

    let exit_pos = Vec2::new(
        new_pos.x + textures.new_game.width() / 2. - textures.exit.width() / 2.,
        280.,
    );
    let exit_x = mouse_position().0 - exit_pos.x;
    let exit_y = mouse_position().1 - exit_pos.y;
    let exit_in = exit_x < textures.exit.width()
        && exit_x > 0.
        && exit_y > 0.
        && exit_y < textures.exit.height();
    draw_texture(
        textures.exit,
        exit_pos.x,
        exit_pos.y,
        if exit_in { LIGHTGRAY } else { WHITE },
    );

    if new_in && is_mouse_button_pressed(MouseButton::Left) {
        *scene = 1;
    } else if exit_in && is_mouse_button_pressed(MouseButton::Left) {
        *exit = true;
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

    if *kind > n + 2 {
        *kind = 1;
    } else if *kind < 1 {
        *kind = n + 2;
    }
}

fn draw_icon(kind: i8, textures: &Textures, wall: &bool, floor: &Map) {
    if *wall {
        let params = DrawTextureParams {
            dest_size: Some(macroquad::prelude::Vec2::new(30., 48.)),
            source: Some(Rect::new(kind as f32 * 15., 0., 15., 24.)),
            ..Default::default()
        };
        draw_texture_ex(textures.walls, 10., 10., WHITE, params);
    } else {
        let n = (kind - (textures.floors.width() / 15.) as i8) as usize;
        if n < floor.animated.len() {
            let params = DrawTextureParams {
                dest_size: Some(macroquad::prelude::Vec2::new(30., 30.)),
                source: Some(Rect::new(n as f32 * 15., 0., 15., 15.)),
                ..Default::default()
            };
            draw_texture_ex(
                floor.animated[n].animations[0].texture,
                10.,
                10.,
                WHITE,
                params,
            );
        } else {
            let params = DrawTextureParams {
                dest_size: Some(macroquad::prelude::Vec2::new(30., 30.)),
                source: Some(Rect::new(kind as f32 * 15., 0., 15., 15.)),
                ..Default::default()
            };
            draw_texture_ex(textures.floors, 10., 10., WHITE, params);
        }
    }
}

fn in_game(
    kind: &mut i8,
    walls: &mut Map,
    floors: &mut Map,
    camera: &mut Camera,
    textures: &Textures,
    wall: &mut bool,
    player: &mut Player,
    enemies: &mut Buffalo,
) {
    clear_background(DARKGRAY);
    edit_map(kind, walls, floors, camera, textures, wall);
    player.update(camera, walls, floors);
    enemies.update(camera, walls);
    floors.update();
    camera.update(player);
    floors.draw(textures, camera);
    walls.draw(textures, camera);
    enemies.draw(textures, camera, walls);
    player.draw(textures, camera, walls);
    draw_icon(*kind, textures, wall, floors);
    draw_text(&get_fps().to_string(), 10., 80., 40., WHITE);
}
