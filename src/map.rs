use macroquad::prelude::{draw_texture_ex, DrawTextureParams, Rect, WHITE};
use std::fs;
use std::io::Write;

use crate::animation::*;
use crate::camera::Camera;
use crate::textures::Textures;
use crate::vecs::Vec2;

#[derive(Clone)]
pub struct Tile {
    pub kind: u8,
}

impl Tile {
    pub fn new(kind: u8) -> Self {
        Tile { kind }
    }
}

pub struct Map {
    pub vec: Vec<Vec<Tile>>,
    pub width: usize,
    pub height: usize,
    pub size: f32,
    pub wall: bool,
    pub animated: Vec<AnimatedSprite>,
    pub chest: AnimatedSprite,
}

impl Map {
    pub fn new(width: usize, height: usize, size: f32, wall: bool, textures: &Textures) -> Self {
        let vec = vec![vec![Tile::new(1); width]; height];

        let water_1 = Animation {
            cur_frame: 0,
            frames: 5,
            height: 15,
            width: 15,
            texture: textures.water_1,
            rect: Rect {
                x: 0.,
                y: 0.,
                w: 15.,
                h: 15.,
            },
        };
        let water_2 = Animation {
            cur_frame: 0,
            frames: 5,
            height: 15,
            width: 15,
            texture: textures.water_2,
            rect: Rect {
                x: 0.,
                y: 0.,
                w: 15.,
                h: 15.,
            },
        };
        let water_1 = AnimatedSprite {
            animations: vec![water_1],
            cur_animation: 0,
            dur: 0.3,
            playing: true,
            time: 0.,
        };
        let water_2 = AnimatedSprite {
            animations: vec![water_2],
            cur_animation: 0,
            dur: 0.3,
            playing: true,
            time: 0.,
        };

        let chest = Animation {
            cur_frame: 0,
            frames: 5,
            height: 25,
            width: 25,
            texture: textures.chest_open,
            rect: Rect {
                x: 0.,
                y: 0.,
                w: 25.,
                h: 25.,
            },
        };
        let chest = AnimatedSprite {
            animations: vec![chest],
            cur_animation: 0,
            dur: 0.3,
            playing: true,
            time: 0.,
        };
        Map {
            vec,
            width,
            height,
            size,
            wall,
            animated: vec![water_1, water_2],
            chest,
        }
    }

    pub async fn from_file(path: &str, size: f32, wall: bool, textures: &Textures) -> Self {
        let string = macroquad::file::load_string(path).await.unwrap();
        let mut vec: Vec<Vec<Tile>> = Vec::new();
        let mut row: Vec<Tile> = Vec::new();
        for c in string.chars() {
            if c.is_numeric() {
                row.push(Tile::new(c as u8 - b'0'));
            }

            if c == '\n' {
                vec.push(row.clone());
                for i in (0..row.len()).rev() {
                    row.remove(i);
                }
            }
        }
        let water_1 = Animation {
            cur_frame: 0,
            frames: 5,
            height: 15,
            width: 15,
            texture: textures.water_1,
            rect: Rect {
                x: 0.,
                y: 0.,
                w: 15.,
                h: 15.,
            },
        };
        let water_2 = Animation {
            cur_frame: 0,
            frames: 5,
            height: 15,
            width: 15,
            texture: textures.water_2,
            rect: Rect {
                x: 0.,
                y: 0.,
                w: 15.,
                h: 15.,
            },
        };
        let water_1 = AnimatedSprite {
            animations: vec![water_1],
            cur_animation: 0,
            dur: 0.3,
            playing: true,
            time: 0.,
        };
        let water_2 = AnimatedSprite {
            animations: vec![water_2],
            cur_animation: 0,
            dur: 0.3,
            playing: true,
            time: 0.,
        };

        let chest = Animation {
            cur_frame: 0,
            frames: 5,
            height: 25,
            width: 25,
            texture: textures.chest_open,
            rect: Rect {
                x: 0.,
                y: 0.,
                w: 25.,
                h: 25.,
            },
        };
        let chest = AnimatedSprite {
            animations: vec![chest],
            cur_animation: 0,
            dur: 0.3,
            playing: true,
            time: 0.,
        };
        Map {
            width: vec[0].len(),
            height: vec.len(),
            vec,
            size,
            wall,
            animated: vec![water_1, water_2],
            chest,
        }
    }

    pub fn to_file(&self) {
        let mut map = String::new();
        for row in &self.vec {
            for tile in row {
                map += &tile.kind.to_string();
            }
            map += "\n";
        }
        println!("{}", map);

        let path = if self.wall {
            "assets/world-data/walls.txt"
        } else {
            "assets/world-data/floors.txt"
        };
        let mut output = fs::File::create(path).unwrap();
        write!(output, "{}", map).unwrap();
    }

    pub fn draw(&self, textures: &Textures, camera: &Camera) {
        let n_floors = (textures.floors.width() / 15.) as u8;
        let n_walls = (textures.walls.width() / 15.) as u8;

        for (y, row) in self.vec.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                if self.wall && tile.kind < n_walls {
                    let params = DrawTextureParams {
                        dest_size: Some(macroquad::prelude::Vec2::new(
                            self.size * camera.zoom,
                            (self.size + 9. * 6.) * camera.zoom,
                        )),
                        source: Some(Rect::new(tile.kind as f32 * 15., 0., 15., 24.)),
                        ..Default::default()
                    };

                    draw_texture_ex(
                        textures.walls,
                        x as f32 * self.size * camera.zoom + camera.pos.x,
                        (y as f32 * self.size - 9. * 6.) * camera.zoom + camera.pos.y,
                        WHITE,
                        params,
                    );
                } else if self.wall {
                    let kind = (tile.kind - n_walls) as usize;
                    if kind == 0 {
                        let pos = Vec2::new(x as f32 * self.size, y as f32 * self.size);
                        self.chest.draw(&pos, &3.6, &false, camera);
                    } else if kind == 1 {
                        let size = 15. * 4.;
                        let pos = Vec2::new(
                            x as f32 * self.size + size / 4.,
                            y as f32 * self.size - size / 1.9,
                        );
                        let params = DrawTextureParams {
                            dest_size: Some(macroquad::prelude::Vec2::new(
                                size * camera.zoom,
                                size * 1.9 * camera.zoom,
                            )),
                            ..Default::default()
                        };

                        draw_texture_ex(
                            textures.statue,
                            pos.x * camera.zoom + camera.pos.x,
                            pos.y * camera.zoom + camera.pos.y,
                            WHITE,
                            params,
                        );
                    }
                } else if !self.wall && tile.kind < n_floors {
                    let params = DrawTextureParams {
                        dest_size: Some(macroquad::prelude::Vec2::new(
                            self.size * camera.zoom,
                            self.size * camera.zoom,
                        )),
                        source: Some(Rect::new(tile.kind as f32 * 15., 0., 15., 15.)),
                        ..Default::default()
                    };

                    draw_texture_ex(
                        textures.floors,
                        (x as f32 * self.size) * camera.zoom + camera.pos.x,
                        (y as f32 * self.size) * camera.zoom + camera.pos.y,
                        WHITE,
                        params,
                    );
                } else if !self.wall {
                    let kind = (tile.kind - n_floors) as usize;

                    if kind < self.animated.len() {
                        let pos = Vec2::new(x as f32 * self.size, y as f32 * self.size);
                        self.animated[kind].draw(&pos, &6., &false, camera);
                    }
                }
            }
        }
    }

    pub fn update(&mut self) {
        for sprite in &mut self.animated {
            sprite.update();
        }
        self.chest.update();
    }
}
