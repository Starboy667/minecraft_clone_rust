use crate::chunk::ChunkId;

pub fn distance(a: (f32, f32), b: &ChunkId) -> i32 {
    let x_dif = a.0 - b.get_pos().0 as f32;
    let z_dif = a.1 - b.get_pos().1 as f32;
    (x_dif.abs() + z_dif.abs()) as i32
}

pub fn block_uv(block: usize, atlas_size: usize) -> Vec<[f32; 2]> {
    let y = block / atlas_size;
    let x = block - y * atlas_size;
    let uv_off = 1.0 / atlas_size as f32;
    let uv_x_0 = (x as f32 + 0.02) * uv_off;
    let uv_x_1 = (x as f32 + 0.98) * uv_off;
    let uv_y_0 = (y as f32 + 0.02) * uv_off;
    let uv_y_1 = (y as f32 + 0.98) * uv_off;
    vec![
        [uv_x_0, uv_y_1],
        [uv_x_1, uv_y_1],
        [uv_x_1, uv_y_0],
        [uv_x_0, uv_y_0],
    ]
}
