use bevy::{
    app::{Plugin, Update},
    asset::{Assets, Handle},
    math::{Vec2, Vec3},
    prelude::{Commands, Mesh, Query, ResMut, Resource, Transform, With},
    utils::HashMap,
};
use noise::{Fbm, MultiFractal, NoiseFn, Perlin};

use crate::{
    chunk::{gen_chunk, Chunk},
    custom_mesh::gen_visible_faces,
    player::{self, Player},
};

// NOISE SETTINGS
pub const CHUNK_SIZE: usize = 16;
pub const CHUNK_HEIGHT: usize = 256;
pub const HEIGHT_OFFSET: i16 = 60;
pub const HEIGHT_INTENSITY: f32 = 0.5;
pub const NOISE_SCALE: Vec2 = Vec2::ONE;
pub const NOISE_OFFSET: Vec2 = Vec2::ZERO;

pub const RENDER_DISTANCE: usize = 10;

#[derive(Resource)]
pub struct World {
    // pub width: i32,
    // pub height: i32,
    // pub depth: i32,
    // update: bool,
    // cubes: Vec<Vec<Vec<usize>>>,
    active_chunks: HashMap<(i32, i32), Mesh>,
    render_distance: u8,
    update: bool,
    // player pos = camera pos
}

// fn height_map(width: i32, height: i32) -> Vec<Vec<f32>> {
//     let mut map = vec![];
//     let scale = 0.1;
//     // let persistence = 0.5;
//     let octaves = 16;
//     // let lacunarity: f64 = 2.0;
//     let frequency: f64 = 0.05261;

//     let fbm: Fbm<Perlin> = Fbm::<Perlin>::new(13)
//         .set_octaves(octaves)
//         // .set_persistence(persistence)
//         // .set_lacunarity(lacunarity)
//         .set_frequency(frequency);

//     for y in 0..height {
//         let mut t = vec![];
//         for x in 0..width {
//             t.push(fbm.get([x as f64 * scale, y as f64 * scale]) as f32);
//         }
//         map.push(t);
//     }
//     map
// }

// fn scale_noise_value(noise_val: f64, min_height: usize, max_height: usize) -> usize {
//     let height_range = max_height - min_height;
//     let scaled_height = (noise_val + 1.0) / 2.0 * height_range as f64; // Normalize to 0.0 to 1.0 and scale
//     (scaled_height as usize) + min_height
// }

impl World {
    pub fn new() -> Self {
        // let width = 16;
        // let height = 40;
        // let depth = 16;
        // let mut cubes = vec![];
        // let mut chunks = vec![];

        // let hmap = height_map(width, depth);
        // let min_height = 0;
        // let max_height = height;

        // for _y in 0..=max_height {
        //     let mut layer = vec![];
        //     for _z in 0..width {
        //         let column = vec![0; depth as usize];
        //         layer.push(column);
        //     }
        //     cubes.push(layer);
        // }

        // for z in 0..width {
        //     for x in 0..depth {
        //         let height_val = scale_noise_value(
        //             hmap[x as usize][z as usize] as f64,
        //             min_height,
        //             max_height as usize,
        //         );
        //         for y in 0..height_val {
        //             cubes[y][z as usize][x as usize] = 1;
        //         }
        //     }
        // }
        // chunks.push(gen_visible_faces(&cubes));
        World {
            active_chunks: HashMap::default(),
            render_distance: RENDER_DISTANCE as u8,
            update: true,
        }
    }

    pub fn set_size(&mut self, height: i32, width: i32, depth: i32) {
        // let mut cubes = vec![];
        // let mut chunks = vec![];
        // let hmap = height_map(width, depth);
        // let min_height = 0;
        // let max_height = height;

        // for _y in 0..=max_height {
        //     let mut layer = vec![];
        //     for _z in 0..width {
        //         let column = vec![0; depth as usize];
        //         layer.push(column);
        //     }
        //     cubes.push(layer);
        // }

        // for z in 0..width {
        //     for x in 0..depth {
        //         let height_val = scale_noise_value(
        //             hmap[x as usize][z as usize] as f64,
        //             min_height,
        //             max_height as usize,
        //         );
        //         for y in 0..height_val {
        //             cubes[y][z as usize][x as usize] = 1;
        //         }
        //     }
        // }
        // chunks.push(gen_visible_faces(&cubes));
        // self.update = true;
        // self.width = width;
        // self.height = height;
        // self.depth = depth;
        // self.cubes = cubes;
        // self.chunks = chunks;
    }
}

fn gen_data(
    // mesh_query: Query<&Handle<Mesh>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut this: ResMut<World>,
    player_query: Query<&Transform, With<Player>>,
) {
    // let mut coords_to_remove = vec![];
    let player_pos = player_query.get_single().unwrap().translation;
    let player_chunk = (
        player_pos.x / CHUNK_SIZE as f32,
        player_pos.z / CHUNK_SIZE as f32,
    );
    let mut x = (player_chunk.0 - this.render_distance as f32) as usize;
    // for (coords, _) in this.active_chunks.iter() {
    //     coords_to_remove.push(coords);
    // }
    let mut coords_to_remove: Vec<(i32, i32)> = this.active_chunks.keys().cloned().collect();
    // if this.active_chunks.len() == 0 {
    //     this.update = true;
    // }
    while x <= (player_chunk.0 + this.render_distance as f32) as usize {
        let mut z = (player_chunk.1 - this.render_distance as f32) as usize;
        while z <= (player_chunk.1 + this.render_distance as f32) as usize {
            if !this.active_chunks.contains_key(&(x as i32, z as i32)) {
                this.active_chunks.insert(
                    (x as i32, z as i32),
                    gen_chunk(Vec2::new(x as f32, z as f32)),
                );
            }
            if let Some(index) = coords_to_remove
                .iter()
                .position(|value| *value == (x as i32, z as i32))
            {
                coords_to_remove.swap_remove(index);
            }
            z += 1;
        }
        x += 1;
    }
    if coords_to_remove.len() > 0 {
        this.update = true;
    }
    for coords in coords_to_remove {
        if let Some(_) = this.active_chunks.remove(&coords) {
            // let Some(mut entity) = commands.get_entity(e);
            // commands.despawn(entity);
            // meshes.remove(mesh);
        }
    }
}

fn render_mesh(
    mut meshes: ResMut<Assets<Mesh>>,
    mut this: ResMut<World>,
    mut mesh_query: Query<(&Handle<Mesh>, &mut Transform), With<Chunk>>,
) {
    if !this.update {
        return;
    }
    // println!("Rendering chunks {:?}", this.active_chunks.len());
    // println!("meshes {:?}", mesh_query.iter().count());
    for ((mesh_handle, mut transf), (chunk_coords, new_mesh)) in
        mesh_query.iter_mut().zip(this.active_chunks.iter())
    {
        if let Some(mesh) = meshes.get_mut(mesh_handle) {
            // *mesh = gen_visible_faces(&this.cubes);
            *mesh = new_mesh.clone();
            transf.translation = Vec3::new(
                chunk_coords.0 as f32 * CHUNK_SIZE as f32,
                0.0,
                chunk_coords.1 as f32 * CHUNK_SIZE as f32,
            );
            // println!("Rendering chunk at {:?}", transf.translation);
        } else {
            panic!("Mesh not found");
        }
    }
    // this.update = false;
}

pub struct WorldPlugin;
impl Plugin for WorldPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(World::new())
            // .add_systems(Startup, setup)
            .add_systems(Update, (gen_data, render_mesh));
    }
}
