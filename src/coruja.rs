use std::f32::consts::PI;

use macroquad::prelude::{draw_texture_ex, get_frame_time, rand, DrawTextureParams, Rect, WHITE};

use crate::animation::*;
use crate::camera::Camera;
use crate::map::*;
use crate::textures::Textures;
use crate::vecs::*;

pub struct Coruja {
    pub pos: Vec2,
    pub size: f32,
    pub sprite: AnimatedSprite,
    pub speed: f32,
    pub flipped: bool,
    pub real_size: Vec<f32>,
    pub dir: Vec<i8>,
    pub hit: Vec<Vec2>,
    pub balls: Vec<Ball>,
    pub time: f32,
    pub cooldown: bool,
}

impl Coruja {
    pub fn new(pos: Vec2, textures: &Textures, size: f32, speed: f32) -> Self {
        let idle = Animation {
            cur_frame: 0,
            frames: 8,
            width: 33,
            height: 40,
            rect: Rect::new(0., 0., 33., 40.),
            texture: textures.coruja_idle,
        };

        let attack = Animation {
            cur_frame: 0,
            frames: 8,
            width: 34,
            height: 43,
            rect: Rect::new(0., 0., 34., 43.),
            texture: textures.coruja_attack,
        };
        let hurt = Animation {
            cur_frame: 0,
            frames: 8,
            width: 34,
            height: 40,
            rect: Rect::new(0., 0., 34., 40.),
            texture: textures.coruja_hurt,
        };

        let animations = vec![idle, attack, hurt];

        let sprite = AnimatedSprite {
            animations: animations.clone(),
            cur_animation: 0,
            dur: 0.2,
            playing: true,
            time: 0.,
        };
        let hit = vec![
            pos,
            Vec2::new(
                animations[0].width as f32,
                size * animations[0].height as f32,
            ),
        ];

        Self {
            pos,
            size,
            sprite,
            speed,
            flipped: false,
            real_size: vec![
                animations[0].width as f32,
                size * animations[0].height as f32,
            ],
            dir: vec![0, 0],
            hit,
            balls: Vec::new(),
            time: 0.,
            cooldown: true
        }
    }

    pub fn draw(&self, textures: &Textures, camera: &Camera, walls: &Map) {
        let size = self.size * self.sprite.animations[self.sprite.cur_animation].height as f32;
        let pos = [
            ((self.pos.x / walls.size) as usize - 1),
            (((self.pos.y + size) / walls.size) as usize - 1),
        ];
        let n_walls = textures.walls.width() / 15.;

        //draw walls close to player
        for y in 0..3 {
            for x in 0..3 {
                let kind: f32;
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
                if kind < n_walls {
                    draw_texture_ex(
                        textures.walls,
                        (pos[0] + x) as f32 * walls.size * camera.zoom + camera.pos.x,
                        ((pos[1] + y) as f32 * walls.size + 3. * 6.) * camera.zoom + camera.pos.y,
                        WHITE,
                        params2,
                    );
                }
            }
        }

        //draw player
        self.sprite
            .draw(&self.pos, &self.size, &self.flipped, camera);

        //draw walls close to player
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
                if kind < n_walls {
                    draw_texture_ex(
                        textures.walls,
                        (pos[0] + x) as f32 * walls.size * camera.zoom + camera.pos.x,
                        ((pos[1] + y) as f32 * walls.size - 9. * 6.) * camera.zoom + camera.pos.y,
                        WHITE,
                        params1,
                    );
                }

                if kind2 != 0. {
                    let params1 = DrawTextureParams {
                        dest_size: Some(macroquad::prelude::Vec2::new(
                            walls.size * camera.zoom,
                            (walls.size - 3. * 6.) * camera.zoom,
                        )),
                        source: Some(Rect::new(kind2 * 15., 0., 15., 12.)),
                        ..Default::default()
                    };
                    if kind2 < n_walls {
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

        for ball in &self.balls {
            ball.draw(camera);
        }
    }

    pub fn movement(&mut self, walls: &Map, floors: &Map) {
        let x: i32 = rand::gen_range(-1, 1);
        let x = x as i8;
        let y: i32 = rand::gen_range(-1, 1);
        let y = y as i8;
        self.dir = vec![x, y];
        let mut speed = self.speed;

        //fix double speed when moving diagonally
        if x.abs() > 0 && y.abs() > 0 {
            speed /= 1.5;
        }
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
        //collision
        let size = [
            self.sprite.animations[self.sprite.cur_animation].width as f32,
            self.size * self.sprite.animations[self.sprite.cur_animation].height as f32,
        ];

        let next_x = self.pos.x + x as f32 * speed + if x > 0 { size[0] * 2. } else { 0. };
        let in_x: bool;
        let x1 = (next_x / walls.size) as usize;
        let y1 = (self.pos.y / walls.size) as usize;
        let y2 = ((self.pos.y + size[1]) / walls.size) as usize;

        in_x = x1 < walls.width
            && ((y1 < walls.height && walls.vec[y1][x1].kind != 0)
                || (y2 < walls.height && walls.vec[y2][x1].kind != 0)
                || (y1 < floors.height
                    && (floors.vec[y1][x1].kind == 7 || floors.vec[y1][x1].kind == 6))
                || (y2 < floors.height
                    && (floors.vec[y2][x1].kind == 7 || floors.vec[y2][x1].kind == 6)));

        let next_y = self.pos.y + y as f32 * speed + if y > 0 { size[1] } else { 0. };
        let in_y: bool;
        let y1 = (next_y / walls.size) as usize;
        let x1 = (self.pos.x / walls.size) as usize;
        let x2 = ((self.pos.x + size[0]) / walls.size) as usize;

        in_y = y1 < walls.height
            && ((x1 < walls.width && walls.vec[y1][x1].kind != 0)
                || (x2 < walls.width && walls.vec[y1][x2].kind != 0)
                || (x1 < floors.width
                    && (floors.vec[y1][x1].kind == 7 || floors.vec[y1][x1].kind == 6))
                || (x2 < floors.width
                    && (floors.vec[y1][x2].kind == 7 || floors.vec[y1][x2].kind == 6)));

        if !in_x {
            self.pos.x += x as f32 * speed;
        }
        if !in_y {
            self.pos.y += y as f32 * speed;
        }
    }

    pub fn update(&mut self, walls: &Map, floors: &Map, textures: &Textures) {
        self.real_size = vec![
            self.sprite.animations[self.sprite.cur_animation].width as f32,
            self.size * self.sprite.animations[self.sprite.cur_animation].height as f32,
        ];
        self.movement(walls, floors);
        self.sprite.update();
        self.hit = vec![
            self.pos,
            Vec2::new(
                self.sprite.animations[self.sprite.cur_animation].width as f32,
                self.size * self.sprite.animations[self.sprite.cur_animation].height as f32,
            ),
        ];

        self.time += get_frame_time();
        if self.time > 0.3 && !self.cooldown {
                    let new_ball = Ball::new(
            self.pos,
            textures,
            Vec2::new(5., rand::gen_range(0., PI * 2.)),
        );
        self.time = 0.;
        self.balls.push(new_ball);
        }


        for ball in &mut self.balls {
            ball.update();
        }

        self.balls.retain(|ball| {!ball.die});
        
    }
}

pub struct Ball {
    pub pos: Vec2,
    pub time: f32,
    pub velocity: Vec2,
    pub sprite: AnimatedSprite,
    pub die: bool,
}

impl Ball {
    pub fn new(pos: Vec2, textures: &Textures, velocity: Vec2) -> Self {
        let ball = Animation {
            cur_frame: 0,
            frames: 5,
            width: 21,
            height: 21,
            rect: Rect::new(0., 0., 21., 21.),
            texture: textures.coruja_ball,
        };

        let animations = vec![ball];

        let sprite = AnimatedSprite {
            animations,
            cur_animation: 0,
            dur: 0.2,
            playing: true,
            time: 0.,
        };

        Ball {
            pos,
            time: 0.,
            velocity,
            sprite,
            die: false,
        }
    }

    pub fn draw(&self, camera: &Camera) {
        self.sprite.draw(&self.pos, &2., &false, camera);
        println!("{} {}", self.pos.x, self.pos.y);
    }

    pub fn update(&mut self) {
        self.pos = Vec2::new(
            self.pos.x + self.velocity.x * self.velocity.y.cos(),
            self.pos.y + self.velocity.x * self.velocity.y.sin(),
        );
        self.sprite.update();
        self.time += get_frame_time();
        if self.time > 10. {
            self.die = true;
        }
    }
}
