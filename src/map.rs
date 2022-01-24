use std::fs;

use macroquad::prelude::{draw_texture_ex, DrawTextureParams, Rect, Vec2, WHITE};

use crate::camera::Camera;
use crate::textures::Textures;

#[derive(Clone)]
pub struct Tile {
    pub kind: u8,
    pub wall: bool,
}

impl Tile {
    pub fn new(kind: u8, wall: bool) -> Self {
        Tile { kind, wall }
    }
}

pub struct Map {
    pub vec: Vec<Vec<Tile>>,
    pub width: usize,
    pub height: usize,
    pub size: f32,
}

impl Map {
    pub fn new(width: usize, height: usize, size: f32) -> Self {
        let vec = vec![vec![Tile::new(1, false); width]; height];

        Map {
            vec,
            width,
            height,
            size,
        }
    }

    pub fn from_file(path: &str, size: f32) -> Self {
        let string = fs::read_to_string(path).expect("Unable to read level from file.");
        let mut vec: Vec<Vec<Tile>> = Vec::new();
        let mut row: Vec<Tile> = Vec::new();
        for c in string.chars() {
            if c.is_numeric() {
                row.push(Tile::new(c as u8 - '0' as u8, false));
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
        }
    }

    pub fn draw(&self, textures: &Textures, camera: &Camera) {
        for (y, row) in self.vec.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                let params = DrawTextureParams {
                    dest_size: Some(Vec2::new(self.size * camera.zoom, self.size * camera.zoom)),
                    source: Some(Rect::new(tile.kind as f32 * 15., 0., 15., 15.)),
                    ..Default::default()
                };

                draw_texture_ex(
                    textures.floors,
                    (x as f32 * self.size) * camera.zoom + camera.pos.x,
                    (y as f32 * self.size) * camera.zoom + camera.pos.y,
                    WHITE,
                    params.clone(),
                );
            }
        }
    }
}
