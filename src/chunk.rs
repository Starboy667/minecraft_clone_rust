use bevy::{
    math::Vec2,
    prelude::{Component, Mesh},
};
use noise::{Fbm, MultiFractal, NoiseFn, Perlin};
use splines::{Interpolation, Key, Spline};

use crate::{
    block::Blocks,
    constant::{
        CHUNK_HEIGHT, CHUNK_SIZE, HEIGHT_INTENSITY, HEIGHT_OFFSET, NOISE_OCTAVES, NOISE_OFFSET,
        NOISE_PERSISTENCE, NOISE_SCALE, WATER_LEVEL,
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

pub struct TerrainSettings {
    // mesh: Mesh,
    // id: ChunkId,
    CHUNK_SIZE: usize,
    CHUNK_HEIGHT: usize,
    pub a: (f32, f32),
    pub b: (f32, f32),
    pub c: (f32, f32),
    pub d: (f32, f32),
    pub NOISE_OCTAVES: usize,
    pub NOISE_PERSISTENCE: f64,
}

impl TerrainSettings {
    pub fn new() -> Self {
        TerrainSettings {
            // mesh
            // , id
            //      }
            CHUNK_SIZE: 16,
            CHUNK_HEIGHT: 256,
            // a: (-0.15, 0.0),
            // b: (0.45, 50.0),
            // c: (1.25, 150.0),
            // d: (1.0, 150.0),
            a: (-0.45, 10.0),
            b: (-0.20, 50.0),
            c: (0.85, 75.0),
            d: (1.0, 150.0),
            // HEIGHT_OFFSET: 60,
            // HEIGHT_INTENSITY: 5.0,
            // NOISE_SCALE: Vec2::ONE,
            // NOISE_OFFSET: Vec2::ZERO,
            NOISE_OCTAVES: 4,
            NOISE_PERSISTENCE: 0.5,
        }
    }
    fn height_map(&self, offset: Vec2) -> Vec<Vec<f32>> {
        let mut map = vec![];

        let fbm: Fbm<Perlin> = Fbm::<Perlin>::new(13)
            .set_octaves(self.NOISE_OCTAVES)
            .set_persistence(self.NOISE_PERSISTENCE);
        // .set_lacunarity(lacunarity)
        // .set_frequency(frequency);
        let JIGGLE: f64 = std::f64::consts::PI - 3.;

        for x in 0..CHUNK_SIZE + 2 {
            let mut t = vec![];
            for y in 0..CHUNK_SIZE + 2 {
                // let perlin_coord_x = self.NOISE_OFFSET.x
                //     + (x as f32 - 1.0 + (offset.x * 16.0)) / CHUNK_SIZE as f32 * self.NOISE_SCALE.x;
                // let perlin_coord_y = self.NOISE_OFFSET.y
                //     + (y as f32 - 1.0 + (offset.y * 16.0)) / CHUNK_SIZE as f32 * self.NOISE_SCALE.y;
                // let formula = (fbm.get([perlin_coord_x as f64, perlin_coord_y as f64])
                //     * self.HEIGHT_INTENSITY as f64
                //     + self.HEIGHT_OFFSET as f64) as f32;

                let height = fbm.get([
                    ((x as f32 - 1.0 + (offset.x * 16.0)) / CHUNK_SIZE as f32) as f64 * JIGGLE,
                    ((y as f32 - 1.0 + (offset.y * 16.0)) / CHUNK_SIZE as f32) as f64 * JIGGLE,
                ]) / 2.
                    + 0.5;
                // let formula = height as f32;
                // t.push(formula);
                // println!("x: {}", (x as f32 - 1.0 + (offset.x * 16.0)));
                // for z in 0..CHUNK_HEIGHT - 1 {
                // let mut tmp = fbm.get([
                //     ((x as f32 - 1.0 + (offset.x * 16.0)) / CHUNK_SIZE as f32) as f64,
                //     ((y as f32 - 1.0 + (offset.y * 16.0)) / CHUNK_SIZE as f32) as f64,
                // ]) as f32;
                // tmp = (tmp - 0.5) * 2.0;
                t.push(height as f32);
                //     - 0.3;
                // println!("z: {}", tmp + 0.3);
                // if tmp >= 0.0 {
                //     break;
                // }
                // if z == CHUNK_HEIGHT - 2 {
                //     t.push(0.0);
                // }
                // }
            }
            map.push(t);
        }
        map
    }

    pub fn gen_chunk(&self, offset: Vec2) -> Mesh {
        let mut cubes: Vec<Vec<Vec<Blocks>>> = vec![];
        let hmap = self.height_map(offset);
        for _y in 0..CHUNK_HEIGHT {
            let mut layer = vec![];
            for _z in 0..CHUNK_SIZE + 2 {
                let column = vec![Blocks::Air; CHUNK_SIZE + 2];
                layer.push(column);
            }
            cubes.push(layer);
        }
        let a = Key::new(self.a.0, self.a.1, Interpolation::Linear);
        let b = Key::new(self.b.0, self.b.1, Interpolation::default());
        let c = Key::new(self.c.0, self.c.1, Interpolation::default());
        let d = Key::new(self.d.0, self.d.1, Interpolation::default());
        let spline = Spline::from_vec(vec![a, b, c, d]);
        for z in 0..CHUNK_SIZE + 2 {
            for x in 0..CHUNK_SIZE + 2 {
                let res = hmap[x as usize][z as usize];
                let mut height_val = 62;
                if let Some(val) = spline.sample(res) {
                    height_val = val as usize;
                }
                for y in 0..height_val {
                    if y < WATER_LEVEL as usize {
                        cubes[y][z as usize][x as usize] = Blocks::Water;
                    } else if y > 100 {
                        cubes[y][z as usize][x as usize] = Blocks::Stone;
                    } else {
                        cubes[y][z as usize][x as usize] = Blocks::Grass;
                    }
                }
            }
        }
        gen_visible_faces(&cubes)
    }
}
