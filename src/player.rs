use macroquad::prelude::{
    draw_texture_ex, get_frame_time, is_key_down, DrawTextureParams, KeyCode, Rect, Texture2D,
    WHITE,
};

use crate::camera::Camera;
use crate::textures::Textures;
use crate::vecs::Vec2;

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

    pub fn draw(&self, camera: &Camera) {
        self.sprite
            .draw(&self.pos, &self.size, &self.flipped, camera);
    }

    pub fn moviment(&mut self, camera: &mut Camera) {
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

            self.pos.x += x as f32 * speed;
            self.pos.y += y as f32 * speed;
        } else {
            camera.pos.x -= x as f32 * 4.;
            camera.pos.y -= y as f32 * 4.;
        }
    }

    pub fn update(&mut self, camera: &mut Camera) {
        self.moviment(camera);
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
