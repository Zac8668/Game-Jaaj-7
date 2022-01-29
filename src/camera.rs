use crate::player::Player;
use crate::vecs::Vec2;
use macroquad::prelude::{screen_height, screen_width};

pub struct Camera {
    pub pos: Vec2,
    pub zoom: f32,
    pub speed: Vec2,
}

impl Camera {
    pub fn update(&mut self, player: &Player) {
        let dist = 5.0;
        let max_dist = dist * 3.0;
        /*let speed_x = -player.speed * player.dir[0] as f32;
        let speed_y = -player.speed * player.dir[1] as f32;

        let mut pos = Vec2::new(
            player.pos.x - self.pos.x + player.real_size[0],
            player.pos.y - self.pos.y + player.real_size[1],
        ); //player pos relative to the camera
        if pos.x.abs() > dist && pos.x.abs() < max_dist {
            self.pos.x += speed_x * (pos.x.abs() / max_dist);
        } else if pos.x.abs() > max_dist {
            self.pos.x += speed_x;
        }
        if pos.y.abs() > dist && pos.y.abs() < max_dist {
            self.pos.y += speed_y * (pos.y.abs() / max_dist);
        } else if pos.y.abs() > max_dist {
            self.pos.y += speed_y;
        } */

        let mut speed = player.speed;

        //fix double speed when moving diagonally
        if player.dir[0].abs() > 0 && player.dir[1].abs() > 0 {
            speed /= 1.5;
        }

        self.speed.x -= speed * player.dir[0] as f32;
        self.speed.y -= speed * player.dir[1] as f32;

        if self.speed.x.abs() > 50. {
            self.speed.x = 50.;
        }
        if self.speed.y.abs() > 50. {
            self.speed.y = 50.;
        }

        self.pos = Vec2::new(
            -player.pos.x * self.zoom + screen_width() / 2. + self.speed.x,
            -player.pos.y * self.zoom + screen_height() / 2. + self.speed.y,
        );
    }
}
