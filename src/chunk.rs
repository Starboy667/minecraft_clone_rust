use bevy::prelude::Mesh;
use noise::{Fbm, Perlin};

use crate::custom_mesh::gen_visible_faces;
// https://youtu.be/esV1-2hB17E?si=8wT3LOnPkvs6ePVL
// mettre à jour liste des chunks actifs (render distance player position)
// delete les chunks plus actifs +  / créer les nouveaux actifs / recycler
// fonction créer chunk (x, y, z) -> mesh

fn height_map(width: i32, height: i32) -> Vec<Vec<f32>> {
    let mut map = vec![];
    let scale = 0.1;
    // let persistence = 0.5;
    let octaves = 16;
    // let lacunarity: f64 = 2.0;
    let frequency: f64 = 0.05261;

    let fbm: Fbm<Perlin> = Fbm::<Perlin>::new(13)
        .set_octaves(octaves)
        // .set_persistence(persistence)
        // .set_lacunarity(lacunarity)
        .set_frequency(frequency);

    for y in 0..height {
        let mut t = vec![];
        for x in 0..width {
            t.push(fbm.get([x as f64 * scale, y as f64 * scale]) as f32);
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

fn gen_chunk() -> Mesh {
    let width = 16;
    let height = 256;
    let depth = 16;
    let mut cubes = vec![];

    let hmap = height_map(width, depth);
    let min_height = 0;
    let max_height = height;

    for _y in 0..=max_height {
        let mut layer = vec![];
        for _z in 0..width {
            let column = vec![0; depth as usize];
            layer.push(column);
        }
        cubes.push(layer);
    }

    for z in 0..width {
        for x in 0..depth {
            let height_val = scale_noise_value(
                hmap[x as usize][z as usize] as f64,
                min_height,
                max_height as usize,
            );
            for y in 0..height_val {
                cubes[y][z as usize][x as usize] = 1;
            }
        }
    }
    gen_visible_faces(&cubes)
}
