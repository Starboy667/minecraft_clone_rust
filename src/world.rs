use bevy::{
    app::{Plugin, Update},
    asset::{Assets, Handle},
    prelude::{Mesh, Query, ResMut, Resource},
};
use noise::{Fbm, MultiFractal, NoiseFn, Perlin};

use crate::custom_mesh::gen_visible_faces;

#[derive(Resource)]
pub struct World {
    pub width: i32,
    pub height: i32,
    pub depth: i32,
    update: bool,
    cubes: Vec<Vec<Vec<usize>>>,
    chunks: Vec<Mesh>,
}

fn height_map(width: i32, height: i32) -> Vec<Vec<f32>> {
    let mut map = vec![];
    let scale = 0.1;
    let octaves = 6;
    let persistence = 0.5;
    let lacunarity: f64 = 2.0;

    let fbm: Fbm<Perlin> = Fbm::<Perlin>::new(13)
        .set_octaves(octaves)
        .set_persistence(persistence)
        .set_lacunarity(lacunarity);

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

impl World {
    pub fn new() -> Self {
        let width = 40;
        let height = 40;
        let depth = 40;
        let mut cubes = vec![];
        let mut chunks = vec![];

        let hmap = height_map(width, depth);
        let min_height = 0;
        let max_height = height;

        for _y in 0..=max_height {
            let mut layer = vec![];
            for _z in 0..depth {
                let column = vec![0; width as usize];
                layer.push(column);
            }
            cubes.push(layer);
        }

        // Place blocks based on the heightmap
        for z in 0..height {
            for x in 0..width {
                let height_val = scale_noise_value(
                    hmap[x as usize][z as usize] as f64,
                    min_height,
                    max_height as usize,
                );
                for y in 0..=height_val {
                    cubes[y][z as usize][x as usize] = 1; // Set this position with a block coordinate
                }
            }
        }
        chunks.push(gen_visible_faces(&cubes));
        World {
            width: width.try_into().unwrap(),
            height,
            depth: depth.try_into().unwrap(),
            cubes,
            chunks,
            update: true,
        }
    }

    // A FIX
    pub fn set_size(&mut self, height: i32, width: i32, depth: i32) {
        let mut cubes = vec![];
        let mut chunks = vec![];
        let hmap = height_map(depth, width);
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

        for z in 0..depth {
            for x in 0..width {
                let height_val = scale_noise_value(
                    hmap[x as usize][z as usize] as f64,
                    min_height,
                    max_height as usize,
                );
                for y in 0..height_val {
                    // Ensure we don't go beyond the available indices.
                    if y < cubes.len()
                        && (z as usize) < cubes[y].len()
                        && (x as usize) < cubes[y][z as usize].len()
                    {
                        cubes[y][z as usize][x as usize] = 1;
                    }
                }
            }
        }
        chunks.push(gen_visible_faces(&cubes));
        self.update = true;
        self.width = width;
        self.height = height;
        self.depth = depth;
        self.cubes = cubes;
        self.chunks = chunks;
    }
}

fn update(
    mesh_query: Query<&Handle<Mesh>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut this: ResMut<World>,
) {
    if !this.update {
        return;
    }
    let mesh_handle = mesh_query.get_single().expect("Query not successful");
    if let Some(mesh) = meshes.get_mut(mesh_handle) {
        *mesh = gen_visible_faces(&this.cubes);
        this.update = false;
    } else {
        panic!("Mesh not found");
    }
}

pub struct WorldPlugin;
impl Plugin for WorldPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(World::new())
            .add_systems(Update, update);
    }
}
