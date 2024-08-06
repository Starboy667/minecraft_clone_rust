use bevy::{
    app::{Plugin, Update},
    asset::{Assets, Handle},
    prelude::{Mesh, Query, ResMut, Resource},
};

use crate::custom_mesh::gen_visible_faces;

#[derive(Resource)]
pub struct World {
    pub width: i32,
    pub height: i32,
    pub depth: i32,
    update: bool,
    cubes: Vec<Vec<Vec<(i32, i32, i32)>>>,
    chunks: Vec<Mesh>,
}

impl World {
    pub fn new() -> Self {
        let width = 5;
        let height = 2;
        let depth = 2;
        let mut cubes = vec![];
        let mut chunks = vec![];
        for y in 0..height {
            let mut t = vec![];
            for z in 0..depth {
                let mut m = vec![];
                for x in 0..width {
                    m.push((x, y, z));
                }
                t.push(m);
            }
            cubes.push(t);
        }
        chunks.push(gen_visible_faces(&cubes));
        World {
            width,
            height,
            depth,
            cubes,
            chunks,
            update: true,
        }
    }

    pub fn set_size(&mut self, height: i32, width: i32, depth: i32) {
        let mut cubes = vec![];
        let mut chunks = vec![];
        for y in 0..height {
            let mut t = vec![];
            for z in 0..depth {
                let mut m = vec![];
                for x in 0..width {
                    m.push((x, y, z));
                }
                t.push(m);
            }
            cubes.push(t);
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
