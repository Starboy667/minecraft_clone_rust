use bevy::{
    math::{Vec2, Vec3},
    prelude::Mesh,
    render::{mesh::Indices, render_asset::RenderAssetUsages, render_resource::PrimitiveTopology},
};

#[derive(Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    Forward,
    Backward,
}

#[derive(Debug)]
pub struct MeshData {
    pos: Vec<[f64; 3]>,
    uv: Vec<[f64; 2]>,
    normal: Vec<[f64; 3]>,
    indices: Vec<u32>,
}

pub fn gen_visible_faces(cubes: &Vec<Vec<Vec<usize>>>) -> Mesh {
    let mut visible_cubes: Vec<MeshData> = Vec::new();
    let mut cube_count = 0;

    for y in 0..cubes.len() {
        for z in 0..cubes[y].len() {
            for x in 0..cubes[y][z].len() {
                // for direction in [
                //     DirectionB::Y,
                //     DirectionB::NegY,
                //     DirectionB::X,
                //     DirectionB::NegX,
                //     DirectionB::Z,
                //     DirectionB::NegZ,
                // ]
                // .iter()
                // {
                if cubes[y][z][x] == 0 {
                    continue;
                }
                for direction in check_visibility(y, z, x, cubes) {
                    match direction {
                        DirectionB::Y => {
                            visible_cubes.push(create_cube_faces_mesh(
                                Direction::Up,
                                x,
                                y,
                                z,
                                cube_count,
                            ));
                        }
                        DirectionB::NegY => {
                            visible_cubes.push(create_cube_faces_mesh(
                                Direction::Down,
                                x,
                                y,
                                z,
                                cube_count,
                            ));
                        }
                        DirectionB::X => {
                            visible_cubes.push(create_cube_faces_mesh(
                                Direction::Right,
                                x,
                                y,
                                z,
                                cube_count,
                            ));
                        }
                        DirectionB::NegX => {
                            visible_cubes.push(create_cube_faces_mesh(
                                Direction::Left,
                                x,
                                y,
                                z,
                                cube_count,
                            ));
                        }
                        DirectionB::Z => {
                            visible_cubes.push(create_cube_faces_mesh(
                                Direction::Backward,
                                x,
                                y,
                                z,
                                cube_count,
                            ));
                        }
                        DirectionB::NegZ => {
                            visible_cubes.push(create_cube_faces_mesh(
                                Direction::Forward,
                                x,
                                y,
                                z,
                                cube_count,
                            ));
                        }
                    }
                    cube_count += 4;
                }
            }
        }
    }
    add_meshes(visible_cubes)
}

#[derive(Debug)]
enum DirectionB {
    X,
    Y,
    Z,
    NegX,
    NegY,
    NegZ,
}

// FIX CORRESPONDACE X Y Z
fn check_visibility(x: usize, y: usize, z: usize, cubes: &Vec<Vec<Vec<usize>>>) -> Vec<DirectionB> {
    let mut directions: Vec<DirectionB> = Vec::new();

    // Y
    match cubes.get(x) {
        Some(inner_vec) => match inner_vec.get(y + 1) {
            Some(inner_inner_vec) => match inner_inner_vec.get(z) {
                Some(val) => {
                    if *val == 0 {
                        directions.push(DirectionB::Z);
                    }
                }
                None => directions.push(DirectionB::Z),
            },
            None => directions.push(DirectionB::Z),
        },
        None => directions.push(DirectionB::Z),
    };
    if y == 0 {
        directions.push(DirectionB::NegZ);
    } else {
        match cubes.get(x) {
            Some(inner_vec) => match inner_vec.get(y - 1) {
                Some(inner_inner_vec) => match inner_inner_vec.get(z) {
                    Some(val) => {
                        if *val == 0 {
                            directions.push(DirectionB::NegZ);
                        }
                    }
                    None => directions.push(DirectionB::NegZ),
                },
                None => {}
            },
            None => {}
        };
    }

    // X
    match cubes.get(x + 1) {
        Some(inner_vec) => match inner_vec.get(y) {
            Some(inner_inner_vec) => match inner_inner_vec.get(z) {
                Some(val) => {
                    if *val == 0 {
                        directions.push(DirectionB::Y);
                    }
                }
                None => directions.push(DirectionB::Y),
            },
            None => directions.push(DirectionB::Y),
        },
        None => directions.push(DirectionB::Y),
    };
    if x == 0 {
        directions.push(DirectionB::NegY);
    } else {
        match cubes.get(x - 1) {
            Some(inner_vec) => match inner_vec.get(y) {
                Some(inner_inner_vec) => match inner_inner_vec.get(z) {
                    Some(val) => {
                        if *val == 0 {
                            directions.push(DirectionB::Y);
                        }
                    }
                    None => directions.push(DirectionB::Y),
                },
                None => {}
            },
            None => {}
        };
    }

    // Z
    match cubes.get(x) {
        Some(inner_vec) => match inner_vec.get(y) {
            Some(inner_inner_vec) => match inner_inner_vec.get(z + 1) {
                Some(val) => {
                    if *val == 0 {
                        directions.push(DirectionB::X);
                    }
                }
                None => directions.push(DirectionB::X),
            },
            None => {}
        },
        None => {}
    };
    if z == 0 {
        directions.push(DirectionB::NegX);
    } else {
        match cubes.get(x) {
            Some(inner_vec) => match inner_vec.get(y) {
                Some(inner_inner_vec) => match inner_inner_vec.get(z - 1) {
                    Some(val) => {
                        if *val == 0 {
                            directions.push(DirectionB::NegX);
                        }
                    }
                    None => directions.push(DirectionB::NegX),
                },
                None => {}
            },
            None => {}
        };
    }
    directions
}

pub fn add_meshes(data: Vec<MeshData>) -> Mesh {
    let mut pos = Vec::new();
    let mut uv = Vec::new();
    let mut normal = Vec::new();
    let mut indices = Vec::new();
    for face in data.iter() {
        pos.extend(
            face.pos
                .iter()
                .map(|&p| Vec3::new(p[0] as f32, p[1] as f32, p[2] as f32)),
        );
        uv.extend(face.uv.iter().map(|&u| Vec2::new(u[0] as f32, u[1] as f32)));
        normal.extend(
            face.normal
                .iter()
                .map(|&n| Vec3::new(n[0] as f32, n[1] as f32, n[2] as f32)),
        );

        indices.extend(face.indices.iter().map(|&i| i));
    }

    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, pos)
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uv)
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normal)
    .with_inserted_indices(Indices::U32(indices))
}

pub fn create_cube_faces_mesh(
    direction: Direction,
    x: usize,
    y: usize,
    z: usize,
    offset: u32,
) -> MeshData {
    let mut pos = match direction {
        Direction::Up => vec![
            // top (facing towards +y)
            [-0.5, 0.5, -0.5], // vertex with index 0
            [0.5, 0.5, -0.5],  // vertex with index 1
            [0.5, 0.5, 0.5],   // etc. until 23
            [-0.5, 0.5, 0.5],
        ],
        Direction::Down => vec![
            // bottom   (-y)
            [-0.5, -0.5, -0.5],
            [0.5, -0.5, -0.5],
            [0.5, -0.5, 0.5],
            [-0.5, -0.5, 0.5],
        ],
        Direction::Right => vec![
            // right    (+x)
            [0.5, -0.5, -0.5],
            [0.5, -0.5, 0.5],
            [0.5, 0.5, 0.5],
            [0.5, 0.5, -0.5],
        ],
        Direction::Left => vec![
            // left     (-x)
            [-0.5, -0.5, -0.5],
            [-0.5, -0.5, 0.5],
            [-0.5, 0.5, 0.5],
            [-0.5, 0.5, -0.5],
        ],
        Direction::Backward => vec![
            // back     (+z)
            [-0.5, -0.5, 0.5],
            [-0.5, 0.5, 0.5],
            [0.5, 0.5, 0.5],
            [0.5, -0.5, 0.5],
        ],
        Direction::Forward => vec![
            // forward  (-z)
            [-0.5, -0.5, -0.5],
            [-0.5, 0.5, -0.5],
            [0.5, 0.5, -0.5],
            [0.5, -0.5, -0.5],
        ],
    };

    let uv = match direction {
        Direction::Up => vec![
            // Assigning the UV coords for the top side.
            [0.0, 0.2],
            [0.0, 0.0],
            [1.0, 0.0],
            [1.0, 0.2],
        ],
        Direction::Down => vec![
            // Assigning the UV coords for the bottom side.
            [0.0, 0.45],
            [0.0, 0.25],
            [1.0, 0.25],
            [1.0, 0.45],
        ],
        Direction::Right => vec![
            // Assigning the UV coords for the right side.
            [1.0, 0.45],
            [0.0, 0.45],
            [0.0, 0.2],
            [1.0, 0.2],
        ],
        Direction::Left => vec![
            // Assigning the UV coords for the left side.
            [1.0, 0.45],
            [0.0, 0.45],
            [0.0, 0.2],
            [1.0, 0.2],
        ],
        Direction::Backward => vec![
            // Assigning the UV coords for the back side.
            [0.0, 0.45],
            [0.0, 0.2],
            [1.0, 0.2],
            [1.0, 0.45],
        ],
        Direction::Forward => vec![
            // Assigning the UV coords for the forward side.
            [0.0, 0.45],
            [0.0, 0.2],
            [1.0, 0.2],
            [1.0, 0.45],
        ],
    };

    let normal = match direction {
        Direction::Up => vec![
            // Normals for the top side (towards +y)
            [0.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
        ],
        Direction::Down => vec![
            // Normals for the bottom side (towards -y)
            [0.0, -1.0, 0.0],
            [0.0, -1.0, 0.0],
            [0.0, -1.0, 0.0],
            [0.0, -1.0, 0.0],
        ],
        Direction::Right => vec![
            // Normals for the right side (towards +x)
            [1.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
        ],
        Direction::Left => vec![
            // Normals for the left side (towards -x)
            [-1.0, 0.0, 0.0],
            [-1.0, 0.0, 0.0],
            [-1.0, 0.0, 0.0],
            [-1.0, 0.0, 0.0],
        ],
        Direction::Backward => vec![
            // Normals for the back side (towards +z)
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
        ],
        Direction::Forward => vec![
            // Normals for the forward side (towards -z)
            [0.0, 0.0, -1.0],
            [0.0, 0.0, -1.0],
            [0.0, 0.0, -1.0],
            [0.0, 0.0, -1.0],
        ],
    };

    let indices = match direction {
        Direction::Up | Direction::Right | Direction::Backward => vec![
            offset,
            offset + 3,
            offset + 1,
            offset + 1,
            offset + 3,
            offset + 2,
        ],
        Direction::Down | Direction::Left | Direction::Forward => vec![
            offset,
            offset + 1,
            offset + 3,
            offset + 1,
            offset + 2,
            offset + 3,
        ],
    };
    for p in pos.iter_mut() {
        p[0] += x as f64;
        p[1] += y as f64;
        p[2] += z as f64;
    }
    MeshData {
        pos,
        uv,
        normal,
        indices,
    }
}
