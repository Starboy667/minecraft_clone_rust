use bevy::{
    math::Vec2,
    prelude::{Component, Mesh},
};
use noise::{Fbm, MultiFractal, NoiseFn, Perlin};

use crate::{
    custom_mesh::gen_visible_faces,
    world::{CHUNK_HEIGHT, CHUNK_SIZE, HEIGHT_INTENSITY, HEIGHT_OFFSET, NOISE_OFFSET, NOISE_SCALE},
};
// https://youtu.be/esV1-2hB17E?si=8wT3LOnPkvs6ePVL
// mettre à jour liste des chunks actifs (render distance player position)
// delete les chunks plus actifs +  / créer les nouveaux actifs / recycler
// fonction créer chunk (x, y, z) -> mesh

#[derive(Component)]
pub struct Chunk {}

impl Default for Chunk {
    fn default() -> Self {
        Chunk {}
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

    for x in 0..CHUNK_SIZE {
        let mut t = vec![];
        for y in 0..CHUNK_SIZE {
            let PerlinCoordX =
                NOISE_OFFSET.x + (x as f32 + (offset.x * 16.0)) / CHUNK_SIZE as f32 * NOISE_SCALE.x;
            let PerlinCoordY =
                NOISE_OFFSET.y + (y as f32 + (offset.y * 16.0)) / CHUNK_SIZE as f32 * NOISE_SCALE.y;
            // let HeightGen = math
            // Mathf.RoundToInt(
            //     Mathf.PerlinNoise(PerlinCoordX, PerlinCoordY) * HeightIntensity + HeightOffset,
            // );
            t.push(
                (fbm.get([PerlinCoordX as f64, PerlinCoordY as f64]) * HEIGHT_INTENSITY as f64
                    + HEIGHT_OFFSET as f64) as f32,
            )
            // t.push(fbm.get([
            //     (x as f64 + (offset.x as f64 * 16.0)) / 16.0 * scale,
            //     (y as f64 + (offset.y as f64 * 16.0)) / 16.0 * scale,
            // ]) as f32);
        }
        map.push(t);
    }
    map
}

fn scale_noise_value(noise_val: f64, min_height: usize, max_height: usize) -> usize {
    let height_range = max_height - min_height;
    let scaled_height = (noise_val + 1.0) / 2.0 * height_range as f64; // Normalize to 0.0 to 1.0 and scale
    (scaled_height as usize) + min_height
}

pub fn gen_chunk(offset: Vec2) -> Mesh {
    let mut cubes = vec![];

    let hmap = height_map(offset);
    for _y in 0..CHUNK_HEIGHT {
        let mut layer = vec![];
        for _z in 0..CHUNK_SIZE {
            let column = vec![0; CHUNK_SIZE as usize];
            layer.push(column);
        }
        cubes.push(layer);
    }
    for z in 0..CHUNK_SIZE {
        for x in 0..CHUNK_SIZE {
            // let height_val =
            //     scale_noise_value(hmap[x as usize][z as usize] as f64, 0, CHUNK_HEIGHT);
            let height_val = hmap[x as usize][z as usize] as usize;
            // println!("height_val: {:?}", height_val);
            for y in 0..height_val {
                cubes[y][z as usize][x as usize] = 1;
            }
        }
    }
    gen_visible_faces(&cubes, offset)
}

// fn update(
//     mesh_query: Query<&Handle<Mesh>>,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut this: ResMut<World>,
// player: Query<&Transform, With<Player>>,
// ) {
// if !this.update {
//     return;
// }
// let mesh_handle = mesh_query.get_single().expect("Query not successful");
// if let Some(mesh) = meshes.get_mut(mesh_handle) {
//     *mesh = gen_visible_faces(&this.cubes);
//     this.update = false;
// } else {
//     panic!("Mesh not found");
// }
// }
