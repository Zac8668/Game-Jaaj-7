use macroquad::prelude::{
    draw_texture_ex, get_frame_time, DrawTextureParams, Rect, Texture2D, WHITE, KeyCode, is_key_down
};

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
            texture: textures.idle
        };

        let animations = vec![idle];

        let sprite = AnimatedSprite {
            animations,
            cur_animation: 0,
            dur: 0.2,
            playing: true,
            time: 0.
        };

        Self { pos, size, sprite, speed, flipped: false}
    }

    pub fn draw(&self) {
        self.sprite.draw(&self.pos, &self.size, &self.flipped);
    }

    pub fn moviment(&mut self) {
        if is_key_down(KeyCode::D) && !is_key_down(KeyCode::A) {
            self.pos.x += self.speed;
            self.flipped = false;
        }
        else if is_key_down(KeyCode::A) && !is_key_down(KeyCode::D) {
            self.pos.x -= self.speed;
            self.flipped = true;
        }
        if is_key_down(KeyCode::S) && !is_key_down(KeyCode::W) {
            self.pos.y += self.speed;
        }
        else if is_key_down(KeyCode::W) && !is_key_down(KeyCode::S) {
            self.pos.y -= self.speed;
        }
    }

    pub fn update(&mut self) {
        self.moviment();
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

    pub fn draw(&self, pos: &Vec2, size: &f32, flipped: &bool) {
        let animation = &self.animations[self.cur_animation];

        let params = DrawTextureParams {
            source: Some(animation.rect),
            dest_size: Some(macroquad::prelude::Vec2::new(
                animation.width as f32 * size,
                animation.height as f32 * size,
            )),
            flip_x: *flipped,
            ..Default::default()
        };

        draw_texture_ex(animation.texture, pos.x, pos.y, WHITE, params);
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
