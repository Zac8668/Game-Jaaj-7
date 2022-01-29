use crate::player::Player;
use crate::vecs::Vec2;
use macroquad::prelude::{screen_width, screen_height};

pub struct Camera {
    pub pos: Vec2,
    pub zoom: f32,
}

impl Camera {
    pub fn update(&mut self, player: &Player) {
        /*let dist = 5.0;
        let max_dist = dist * 3.0;
        let speed_x = -player.speed * player.dir[0] as f32;
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
        self.pos = Vec2::new(-player.pos.x + screen_width() / 2. * self.zoom, -player.pos.y + screen_height() / 2. * self.zoom);
        println!("{} {}", self.pos.x, self.pos.y);
    }
}
