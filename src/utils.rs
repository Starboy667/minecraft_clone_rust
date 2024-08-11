use crate::{block::Blocks, chunk::ChunkId, custom_mesh::Direction};

pub fn distance(a: (f32, f32), b: &ChunkId) -> i32 {
    let x_dif = a.0 - b.get_pos().0 as f32;
    let z_dif = a.1 - b.get_pos().1 as f32;
    (x_dif.abs() + z_dif.abs()) as i32
}

pub fn block_uv(block: usize, atlas_size: usize, rotate: Option<u8>) -> Vec<[f32; 2]> {
    let y = block / atlas_size;
    let x = block % atlas_size;
    let uv_off = 1.0 / atlas_size as f32;
    let uv_x_0 = (x as f32 + 0.02) * uv_off;
    let uv_x_1 = (x as f32 + 0.98) * uv_off;
    let uv_y_0 = (y as f32 + 0.02) * uv_off;
    let uv_y_1 = (y as f32 + 0.98) * uv_off;

    let mut coords = vec![
        [uv_x_0, uv_y_1],
        [uv_x_1, uv_y_1],
        [uv_x_1, uv_y_0],
        [uv_x_0, uv_y_0],
    ];

    if let Some(rot) = rotate {
        for _ in 0..rot {
            coords = vec![
                [coords[3][0], coords[3][1]],
                [coords[0][0], coords[0][1]],
                [coords[1][0], coords[1][1]],
                [coords[2][0], coords[2][1]],
            ];
        }
    }

    coords
}

pub fn get_block_index(block: &Blocks, dir: &Direction) -> usize {
    match block {
        Blocks::Air => 40,
        Blocks::Grass => match dir {
            Direction::Up => 0,
            Direction::Down => 2,
            _ => 1,
        },
        Blocks::Dirt => match dir {
            _ => 2,
        },
        Blocks::Stone => 3,
        Blocks::Bedrock => 20,
        Blocks::Water => 8,
    }
}
