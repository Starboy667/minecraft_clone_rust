use bevy::{
    math::Vec2,
    prelude::{Component, Mesh},
};
use noise::{Fbm, NoiseFn, Perlin};

use crate::{
    constant::{
        CHUNK_HEIGHT, CHUNK_SIZE, HEIGHT_INTENSITY, HEIGHT_OFFSET, NOISE_OFFSET, NOISE_SCALE,
    },
    custom_mesh::gen_visible_faces,
};

#[derive(Component, Debug)]
pub struct ChunkId(pub (i32, i32));
impl ChunkId {
    pub fn get_pos(&self) -> (i32, i32) {
        self.0
    }
}

pub struct Chunk {
    // mesh: Mesh,
    // id: ChunkId,
}

impl Chunk {
    pub fn new(id: ChunkId, mesh: Mesh) -> Self {
        Chunk { 
            // mesh
            // , id
         }
    }
}

fn height_map(offset: Vec2) -> Vec<Vec<f32>> {
    let mut map = vec![];
    // let scale = 0.75;
    // let persistence = 0.5;
    // let octaves = 16;
    // let lacunarity: f64 = 2.0;
    // let frequency: f64 = 0.05261;

    let fbm: Fbm<Perlin> = Fbm::<Perlin>::new(13);
    // .set_octaves(octaves)
    // .set_persistence(persistence)
    // .set_lacunarity(lacunarity)
    // .set_frequency(frequency);

    for x in 0..CHUNK_SIZE + 2 {
        let mut t = vec![];
        for y in 0..CHUNK_SIZE + 2 {
            let perlin_coord_x =
                NOISE_OFFSET.x + (x as f32 - 1.0 + (offset.x * 16.0)) / CHUNK_SIZE as f32 * NOISE_SCALE.x;
            let perlin_coord_y =
                NOISE_OFFSET.y + (y as f32 - 1.0 + (offset.y * 16.0)) / CHUNK_SIZE as f32 * NOISE_SCALE.y;
            t.push(
                (fbm.get([perlin_coord_x as f64, perlin_coord_y as f64]) * HEIGHT_INTENSITY as f64
                    + HEIGHT_OFFSET as f64) as f32,
            )
        }
        map.push(t);
    }
    map
}

pub fn gen_chunk(offset: Vec2) -> Mesh {
    let mut cubes = vec![];

    let hmap = height_map(offset);
    for _y in 0..CHUNK_HEIGHT {
        let mut layer = vec![];
        for _z in 0..CHUNK_SIZE + 2 {
            let column = vec![0; CHUNK_SIZE  + 2 as usize];
            layer.push(column);
        }
        cubes.push(layer);
    }
    for z in 0..CHUNK_SIZE + 2 {
        for x in 0..CHUNK_SIZE + 2 {
            let height_val = hmap[x as usize][z as usize] as usize;
            for y in 0..height_val {
                cubes[y][z as usize][x as usize] = 1;
            }
        }
    }
    gen_visible_faces(&cubes)
}