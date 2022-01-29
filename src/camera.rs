use crate::player::Player;
use crate::vecs::Vec2;
use macroquad::prelude::{screen_height, screen_width};

pub struct Camera {
    pub pos: Vec2,
    pub zoom: f32,
    pub speed: Vec2,
    pub speed_limit: Vec2
}

impl Camera {
    pub fn update(&mut self, player: &Player) {
        let mut speed = player.speed;

        //fix double speed when moving diagonally
        if player.dir[0].abs() > 0 && player.dir[1].abs() > 0 {
            speed /= 1.5;
        }

        self.speed.x -= speed * player.dir[0] as f32;
        self.speed.y -= speed * player.dir[1] as f32;

        if self.speed.x.abs() > self.speed_limit.x {
            let mut mult = (self.speed.x > 0.) as i8;
            if mult == 0 {
                mult = -1;
            }
            self.speed.x = self.speed_limit.x * mult as f32;
        }
        if self.speed.y.abs() > self.speed_limit.y {
            let mut mult = (self.speed.y > 0.) as i8;
            if mult == 0 {
                mult = -1;
            }
            self.speed.y = self.speed_limit.y * mult as f32;
        }

        self.pos = Vec2::new(
            -player.pos.x * self.zoom + screen_width() / 2. + self.speed.x,
            -player.pos.y * self.zoom + screen_height() / 2. + self.speed.y,
        );
    }
}
