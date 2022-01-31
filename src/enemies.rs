use crate::camera::Camera;
use crate::coruja::Coruja;
use crate::player::Player;
use crate::textures::Textures;
use crate::Map;

pub struct Enemies {
    pub corujas: Vec<Coruja>,
}

impl Enemies {
    pub fn update(&mut self, walls: &Map, floors: &Map, player: &Player, textures: &Textures) {
        for coruja in &mut self.corujas {
            coruja.update(walls, floors, textures);
            let coruja_hurt = ((coruja.pos.x - player.sword_hit[0].x > player.sword_hit[1].x)
                || (coruja.pos.y - player.sword_hit[0].y > player.sword_hit[1].y))
                || !player.attacking
                || coruja.pos.x < 0.
                || coruja.pos.y < 0.;

            if coruja_hurt {
                coruja.sprite.cur_animation = 2;
            }
        }

        /*self.corujas.retain(|coruja| {
            ((coruja.pos.x - player.sword_hit[0].x > player.sword_hit[1].x)
                || (coruja.pos.y - player.sword_hit[0].y > player.sword_hit[1].y))
                || !player.attacking
                || coruja.pos.x < 0.
                || coruja.pos.y < 0.
        });*/
    }

    pub fn draw(&self, textures: &Textures, camera: &Camera, walls: &Map) {
        for coruja in &self.corujas {
            coruja.draw(textures, camera, walls)
        }
    }

    pub fn attack(&mut self) {}
}
