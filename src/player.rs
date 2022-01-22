use macroquad::prelude::{
    draw_texture_ex, get_frame_time, DrawTextureParams, Rect, Texture2D, WHITE,
};

use crate::textures::Textures;
use crate::vecs::Vec2;

pub struct Player {
    pub pos: Vec2,
    pub size: f32,
    pub sprite: AnimatedSprite,
}

impl Player {
    pub fn new(pos: Vec2, textures: &Textures, size: f32) -> Self {
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

        Self { pos, size, sprite }
    }

    pub fn draw(&self) {
        self.sprite.draw(&self.pos, &self.size);
    }

    pub fn update(&mut self) {
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

    pub fn draw(&self, pos: &Vec2, size: &f32) {
        let animation = &self.animations[self.cur_animation];

        let params = DrawTextureParams {
            source: Some(animation.rect),
            dest_size: Some(macroquad::prelude::Vec2::new(
                animation.width as f32 * size,
                animation.height as f32 * size,
            )),
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
