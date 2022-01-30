use crate::vecs::Vec2;
use crate::Camera;
use macroquad::prelude::{
    draw_texture_ex, get_frame_time, DrawTextureParams, Rect, Texture2D, WHITE,
};

pub struct AnimatedSprite {
    pub animations: Vec<Animation>,
    pub cur_animation: usize,
    pub dur: f32,
    pub time: f32,
    pub playing: bool,
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

#[derive(Clone)]
pub struct Animation {
    pub texture: Texture2D,
    pub width: usize,
    pub height: usize,
    pub frames: usize,
    pub cur_frame: usize,
    pub rect: Rect,
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
