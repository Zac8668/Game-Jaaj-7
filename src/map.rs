use macroquad::prelude::{draw_texture_ex, DrawTextureParams, Rect, Vec2, WHITE};
use std::fs;
use std::io::Write;

use crate::camera::Camera;
use crate::textures::Textures;

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
}

impl Map {
    pub fn new(width: usize, height: usize, size: f32, wall: bool) -> Self {
        let vec = vec![vec![Tile::new(1); width]; height];

        Map {
            vec,
            width,
            height,
            size,
            wall,
        }
    }

    pub fn from_file(path: &str, size: f32, wall: bool) -> Self {
        let string = fs::read_to_string(path).expect("Unable to read level from file.");
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

        Map {
            width: vec[0].len(),
            height: vec.len(),
            vec,
            size,
            wall,
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

        let path = if self.wall { "walls.txt" } else { "floors.txt" };
        let mut output = fs::File::create(path).unwrap();
        write!(output, "{}", map).unwrap();
    }

    pub fn draw(&self, textures: &Textures, camera: &Camera) {
        for (y, row) in self.vec.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                if self.wall {
                    let params = DrawTextureParams {
                        dest_size: Some(Vec2::new(
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
                } else {
                    let params = DrawTextureParams {
                        dest_size: Some(Vec2::new(
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
                }
            }
        }
    }
}
