use bevy::{
    app::{Plugin, Update},
    asset::Assets,
    color::Color,
    math::Vec2,
    pbr::{PbrBundle, StandardMaterial},
    prelude::{default, Commands, Entity, Mesh, Query, Res, ResMut, Resource, Transform, With},
    utils::HashMap,
};

use crate::{
    chunk::{Chunk, ChunkId, TerrainSettings},
    constant::{CHUNK_SIZE, RENDER_DISTANCE},
    player::Player,
    texture::TextureHandles,
    utils::distance,
};

#[derive(Resource)]
pub struct World {
    chunks: HashMap<(i32, i32), Chunk>,
    pub render_distance: u8,
    pub terrain_settings: TerrainSettings,
}

impl World {
    pub fn new() -> Self {
        World {
            chunks: HashMap::default(),
            render_distance: RENDER_DISTANCE as u8,
            terrain_settings: TerrainSettings::new(),
            // update: true,
        }
    }

    pub fn reset(&mut self, commands: &mut Commands, chunks: &mut Query<(Entity, &ChunkId)>) {
        for (entity, pos_chunk) in chunks.iter_mut() {
            commands.entity(entity).despawn();
            self.chunks.remove(&pos_chunk.get_pos());
        }
    }
}

fn gen_data(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut world: ResMut<World>,
    player_query: Query<&Transform, With<Player>>,
    mut chunks: Query<(Entity, &ChunkId)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    texture: Res<TextureHandles>,
) {
    let player_world = player_query.get_single().unwrap().translation;
    let player_chunk = (
        player_world.x / CHUNK_SIZE as f32,
        player_world.z / CHUNK_SIZE as f32,
    );

    // println!("World {:?}", world.chunks.len());
    // println!("chunks {:?}", chunks.iter().count());
    for (entity, pos_chunk) in chunks.iter_mut() {
        let distance = distance(player_chunk, pos_chunk);
        if distance > (world.render_distance * 2) as i32 {
            // println!("Despawning chunk {:?}", pos_chunk.0);
            commands.entity(entity).despawn();
            world.chunks.remove(&pos_chunk.get_pos());
        }
    }

    // let texture_atlas = TextureAtlasLayout::from_grid(UVec2::splat(16), 16, 16, None, None);
    for z in -(world.render_distance as i32)..world.render_distance as i32 {
        for x in -(world.render_distance as i32)..world.render_distance as i32 {
            let x = x + player_chunk.0 as i32;
            let z = z + player_chunk.1 as i32;
            if !world.chunks.contains_key(&(x as i32, z as i32)) {
                let mesh = world
                    .terrain_settings
                    .gen_chunk(Vec2::new(x as f32, z as f32));
                world.chunks.insert((x, z), Chunk {});
                // println!("Spawning chunk {:?}", (x, z));
                commands
                    .spawn(PbrBundle {
                        mesh: meshes.add(mesh),
                        material: materials.add(StandardMaterial {
                            base_color: Color::WHITE,
                            base_color_texture: Some(texture.dirt.clone()),

                            ..default()
                        }),
                        transform: Transform::from_xyz(
                            x as f32 * CHUNK_SIZE as f32,
                            0.0,
                            z as f32 * CHUNK_SIZE as f32,
                        ),
                        ..default()
                    })
                    .insert(ChunkId((x, z)));
            }
        }
    }
}

pub struct WorldPlugin;
impl Plugin for WorldPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(World::new())
            // .add_systems(Startup, setup)
            .add_systems(Update, gen_data);
    }
}
