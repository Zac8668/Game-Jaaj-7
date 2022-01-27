use macroquad::prelude::{
    draw_texture_ex, get_frame_time, is_key_down, DrawTextureParams, KeyCode, Rect, Texture2D,
    WHITE,
};

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
            animations,
            cur_animation: 0,
            dur: 0.2,
            playing: true,
            time: 0.,
        };

        Self {
            pos,
            size,
            sprite,
            speed,
            flipped: false,
        }
    }

    pub fn draw(&self, textures: &Textures, camera: &Camera, walls: &Map) {
        let size = self.size * self.sprite.animations[self.sprite.cur_animation].height as f32;

        let pos = [
            ((self.pos.x / walls.size) as usize - 1),
            (((self.pos.y + size) / walls.size) as usize - 1),
        ];

        for y in 0..3 {
            for x in 0..3 {
                let mut kind: f32;
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

                draw_texture_ex(
                    textures.walls,
                    (pos[0] + x) as f32 * walls.size * camera.zoom + camera.pos.x,
                    ((pos[1] + y) as f32 * walls.size + 3. * 6.) * camera.zoom + camera.pos.y,
                    WHITE,
                    params2,
                );
            }
        }

        self.sprite
            .draw(&self.pos, &self.size, &self.flipped, camera);

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

                draw_texture_ex(
                    textures.walls,
                    (pos[0] + x) as f32 * walls.size * camera.zoom + camera.pos.x,
                    ((pos[1] + y) as f32 * walls.size - 9. * 6.) * camera.zoom + camera.pos.y,
                    WHITE,
                    params1,
                );

                if kind2 != 0. {
                    let params1 = DrawTextureParams {
                        dest_size: Some(macroquad::prelude::Vec2::new(
                            walls.size * camera.zoom,
                            (walls.size - 3. * 6.) * camera.zoom,
                        )),
                        source: Some(Rect::new(kind2 * 15., 0., 15., 12.)),
                        ..Default::default()
                    };

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

    pub fn movement(&mut self, camera: &mut Camera, walls: &Map) {
        let x = is_key_down(KeyCode::D) as i8 + -(is_key_down(KeyCode::A) as i8);
        let y = is_key_down(KeyCode::S) as i8 + -(is_key_down(KeyCode::W) as i8);

        let mut speed = self.speed;

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
            let size = [
                self.sprite.animations[self.sprite.cur_animation].width as f32,
                self.size * self.sprite.animations[self.sprite.cur_animation].height as f32,
            ];
            self.pos.x += x as f32 * speed;
            self.pos.y += y as f32 * speed;
        } else {
            camera.pos.x -= x as f32 * 4.;
            camera.pos.y -= y as f32 * 4.;
            self.sprite.cur_animation = 0;
        }
    }

    pub fn update(&mut self, camera: &mut Camera, walls: &Map) {
        self.movement(camera, walls);
        self.sprite.update();
    }
}

pub struct AnimatedSprite {
    animations: Vec<Animation>,
    cur_animation: usize,
    dur: f32,
    time: f32,
    playing: bool,
}

impl AnimatedSprite {
    pub fn update(&mut self) {
        self.time += get_frame_time();
        self.animations[self.cur_animation].update(&mut self.time, &self.dur, &self.playing);
    }

    pub fn draw(&self, pos: &Vec2, size: &f32, flipped: &bool, camera: &Camera) {
        let animation = &self.animations[self.cur_animation];

        let params = DrawTextureParams {
            source: Some(animation.rect),
            dest_size: Some(macroquad::prelude::Vec2::new(
                animation.width as f32 * size * camera.zoom,
                animation.height as f32 * size * camera.zoom,
            )),
            flip_x: *flipped,
            ..Default::default()
        };

        draw_texture_ex(
            animation.texture,
            pos.x * camera.zoom + camera.pos.x,
            pos.y * camera.zoom + camera.pos.y,
            WHITE,
            params,
        );
    }
}

struct Animation {
    texture: Texture2D,
    width: usize,
    height: usize,
    frames: usize,
    cur_frame: usize,
    rect: Rect,
}

impl Animation {
    pub fn update(&mut self, time: &mut f32, dur: &f32, playing: &bool) {
        if *time + get_frame_time() > *dur {
            *time += get_frame_time() - dur;
            if *playing {
                self.cur_frame += 1;
                if self.cur_frame == self.frames {
                    self.cur_frame = 0;
                }
                self.rect = Rect::new(
                    (self.cur_frame * self.width) as f32,
                    0.,
                    self.width as f32,
                    self.height as f32,
                );
            }
        }
    }
}
