use crate::Textures;
use macroquad::prelude::{
    draw_texture, get_frame_time, is_mouse_button_pressed, Color, MouseButton, Texture2D, WHITE,
};

pub struct History {
    pub quadrinhos: Vec<Texture2D>,
    pub cur: usize,
    pub time: f32,
    pub on_time: f32,
    pub color: Color,
    pub color_sub: f32,
    pub change_color: f32,
}

impl History {
    pub fn new(textures: &Textures) -> Self {
        Self {
            quadrinhos: vec![
                textures.quadrinho_1,
                textures.quadrinho_2,
                textures.quadrinho_3,
            ],
            cur: 0,
            time: 0.,
            on_time: 10.,
            color: WHITE,
            color_sub: 0.01,
            change_color: 0.2,
        }
    }

    pub fn tick(&mut self, scene: &mut i32) {
        if self.cur < self.quadrinhos.len() {
            draw_texture(self.quadrinhos[self.cur], 0., 0., self.color);
            self.time += get_frame_time();
            if self.time >= self.on_time {
                self.color.r -= self.color_sub;
                self.color.g -= self.color_sub;
                self.color.b -= self.color_sub;
                if self.color.r <= self.change_color || is_mouse_button_pressed(MouseButton::Left) {
                    self.color.r = 1.;
                    self.color.g = 1.;
                    self.color.b = 1.;
                    self.time = 0.;
                    self.cur += 1;
                }
            } else if is_mouse_button_pressed(MouseButton::Left) {
                self.time = self.on_time;
            }
        } else {
            *scene += 1;
        }
    }
}
