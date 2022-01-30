use macroquad::prelude::{
    draw_texture_ex, get_frame_time, is_key_down, is_key_pressed, DrawTextureParams, KeyCode, Rect,
    WHITE,
};

use crate::animation::*;
use crate::camera::Camera;
use crate::map::*;
use crate::textures::Textures;
use crate::vecs::*;

pub struct Player {
    pub pos: Vec2,
    pub size: f32,
    pub sprite: AnimatedSprite,
    pub speed: f32,
    pub flipped: bool,
    pub real_size: Vec<f32>,
    pub dir: Vec<i8>,
    pub sword_sprite: AnimatedSprite,
    pub attacking: bool,
}

impl Player {
    pub fn new(pos: Vec2, textures: &Textures, size: f32, speed: f32) -> Self {
        let idle = Animation {
            cur_frame: 0,
            frames: 6,
            width: 26,
            height: 32,
            rect: Rect::new(0., 0., 26., 34.),
            texture: textures.player_idle,
        };

        let walking = Animation {
            cur_frame: 0,
            frames: 2,
            width: 26,
            height: 32,
            rect: Rect::new(0., 0., 26., 34.),
            texture: textures.player_walk,
        };

        let animations = vec![idle, walking];

        let sprite = AnimatedSprite {
            animations: animations.clone(),
            cur_animation: 0,
            dur: 0.2,
            playing: true,
            time: 0.,
        };

        let sword_attack = Animation {
            cur_frame: 0,
            frames: 4,
            width: 16,
            height: 32,
            rect: Rect::new(0., 0., 16., 32.),
            texture: textures.sword_attack,
        };
        let sword_animations = vec![sword_attack];
        let sword_sprite = AnimatedSprite {
            animations: sword_animations,
            cur_animation: 0,
            dur: 0.08,
            playing: true,
            time: 0.,
        };

        Self {
            pos,
            size,
            sprite,
            speed,
            flipped: false,
            real_size: vec![
                animations[0].width as f32,
                size * animations[0].height as f32,
            ],
            dir: vec![0, 0],
            sword_sprite,
            attacking: true,
        }
    }

    pub fn draw(&self, textures: &Textures, camera: &mut Camera, walls: &Map) {
        let size = self.size * self.sprite.animations[self.sprite.cur_animation].height as f32;
        let pos = [
            ((self.pos.x / walls.size) as usize - 1),
            (((self.pos.y + size) / walls.size) as usize - 1),
        ];
        let n_walls = textures.walls.width() / 15.;

        //draw walls close to player
        for y in 0..3 {
            for x in 0..3 {
                let kind: f32;
                if pos[0] + x < walls.width && pos[1] + y < walls.height {
                    kind = walls.vec[pos[1] as usize + y][pos[0] as usize + x].kind as f32;
                } else {
                    kind = 0.;
                }
                let params2 = DrawTextureParams {
                    dest_size: Some(macroquad::prelude::Vec2::new(
                        walls.size * camera.zoom,
                        (walls.size - 3. * 6.) * camera.zoom,
                    )),
                    source: Some(Rect::new(kind * 15., 12., 15., 12.)),
                    ..Default::default()
                };
                if kind < n_walls {
                    draw_texture_ex(
                        textures.walls,
                        (pos[0] + x) as f32 * walls.size * camera.zoom + camera.pos.x,
                        ((pos[1] + y) as f32 * walls.size + 3. * 6.) * camera.zoom + camera.pos.y,
                        WHITE,
                        params2,
                    );
                }
            }
        }

        //draw player
        let sword_pos = Vec2::new(
            if self.flipped {
                self.pos.x - self.real_size[0]
            } else {
                self.pos.x + self.real_size[0] * 2. - 6.
            },
            self.pos.y + 18.,
        );
        self.sprite
            .draw(&self.pos, &self.size, &self.flipped, camera);
        self.sword_sprite
            .draw(&sword_pos, &2., &self.flipped, camera);

        //draw walls close to player
        for y in 0..3 {
            for x in 0..3 {
                let kind: f32;
                if pos[0] + x < walls.width && pos[1] + y < walls.height {
                    kind = walls.vec[pos[1] as usize + y][pos[0] as usize + x].kind as f32;
                } else {
                    kind = 0.;
                }
                let kind2: f32;
                if pos[0] + x < walls.width && pos[1] + y + 1 < walls.height {
                    kind2 = walls.vec[pos[1] as usize + y + 1][pos[0] as usize + x].kind as f32;
                } else {
                    kind2 = 0.;
                }
                let params1 = DrawTextureParams {
                    dest_size: Some(macroquad::prelude::Vec2::new(
                        walls.size * camera.zoom,
                        (walls.size - 3. * 6.) * camera.zoom,
                    )),
                    source: Some(Rect::new(kind * 15., 0., 15., 12.)),
                    ..Default::default()
                };
                if kind < n_walls {
                    draw_texture_ex(
                        textures.walls,
                        (pos[0] + x) as f32 * walls.size * camera.zoom + camera.pos.x,
                        ((pos[1] + y) as f32 * walls.size - 9. * 6.) * camera.zoom + camera.pos.y,
                        WHITE,
                        params1,
                    );
                }

                if kind2 != 0. {
                    let params1 = DrawTextureParams {
                        dest_size: Some(macroquad::prelude::Vec2::new(
                            walls.size * camera.zoom,
                            (walls.size - 3. * 6.) * camera.zoom,
                        )),
                        source: Some(Rect::new(kind2 * 15., 0., 15., 12.)),
                        ..Default::default()
                    };
                    if kind2 < n_walls {
                        draw_texture_ex(
                            textures.walls,
                            (pos[0] + x) as f32 * walls.size * camera.zoom + camera.pos.x,
                            ((pos[1] + y) as f32 * walls.size - 9. * 6. + walls.size) * camera.zoom
                                + camera.pos.y,
                            WHITE,
                            params1,
                        );
                    }
                }
            }
        }
    }

    pub fn movement(&mut self, camera: &mut Camera, walls: &Map, floors: &Map) {
        let x = is_key_down(KeyCode::D) as i8 + -(is_key_down(KeyCode::A) as i8);
        let y = is_key_down(KeyCode::S) as i8 + -(is_key_down(KeyCode::W) as i8);
        self.dir = vec![x, y];
        let mut speed = self.speed * get_frame_time() * 60.;

        //fix double speed when moving diagonally
        if x.abs() > 0 && y.abs() > 0 {
            speed /= 1.5;
        }
        if !is_key_down(KeyCode::LeftShift) {
            match x {
                x if x < 0 => self.flipped = true,
                x if x > 0 => self.flipped = false,
                _ => (),
            }

            if x != 0 || y != 0 {
                self.sprite.cur_animation = 1;
            } else {
                self.sprite.cur_animation = 0;
            }
            //collision
            let size = [
                self.sprite.animations[self.sprite.cur_animation].width as f32,
                self.size * self.sprite.animations[self.sprite.cur_animation].height as f32,
            ];

            let next_x = self.pos.x + x as f32 * speed + if x > 0 { size[0] * 2. } else { 0. };
            let in_x: bool;
            let x1 = (next_x / walls.size) as usize;
            let y1 = (self.pos.y / walls.size) as usize;
            let y2 = ((self.pos.y + size[1]) / walls.size) as usize;

            in_x = x1 < walls.width
                && ((y1 < walls.height && walls.vec[y1][x1].kind != 0)
                    || (y2 < walls.height && walls.vec[y2][x1].kind != 0)
                    || (y1 < floors.height
                        && (floors.vec[y1][x1].kind == 7 || floors.vec[y1][x1].kind == 6))
                    || (y2 < floors.height
                        && (floors.vec[y2][x1].kind == 7 || floors.vec[y2][x1].kind == 6)));

            let next_y = self.pos.y + y as f32 * speed + if y > 0 { size[1] } else { 0. };
            let in_y: bool;
            let y1 = (next_y / walls.size) as usize;
            let x1 = (self.pos.x / walls.size) as usize;
            let x2 = ((self.pos.x + size[0]) / walls.size) as usize;

            in_y = y1 < walls.height
                && ((x1 < walls.width && walls.vec[y1][x1].kind != 0)
                    || (x2 < walls.width && walls.vec[y1][x2].kind != 0)
                    || (x1 < floors.width
                        && (floors.vec[y1][x1].kind == 7 || floors.vec[y1][x1].kind == 6))
                    || (x2 < floors.width
                        && (floors.vec[y1][x2].kind == 7 || floors.vec[y1][x2].kind == 6)));

            if !in_x {
                self.pos.x += x as f32 * speed;
            }
            if !in_y {
                self.pos.y += y as f32 * speed;
            }
        } else {
            camera.pos.x -= x as f32 * 4.;
            camera.pos.y -= y as f32 * 4.;
            self.sprite.cur_animation = 0;
        }
    }

    pub fn update(&mut self, camera: &mut Camera, walls: &Map, floors: &Map) {
        self.real_size = vec![
            self.sprite.animations[self.sprite.cur_animation].width as f32,
            self.size * self.sprite.animations[self.sprite.cur_animation].height as f32,
        ];
        self.movement(camera, walls, floors);
        self.sprite.update();
        if is_key_down(KeyCode::B)
            && self.sword_sprite.playing
            && self.sword_sprite.animations[0].cur_frame == 0
        {
            self.sword_sprite.playing = false;
        } else if is_key_pressed(KeyCode::B) {
            self.sword_sprite.playing = true;
        }
        self.sword_sprite.update();
    }
}
