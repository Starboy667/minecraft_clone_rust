use crate::chunk::ChunkId;

pub fn distance(a: (f32, f32), b: &ChunkId) -> i32 {
    let x_dif = a.0 - b.get_pos().0 as f32;
    let z_dif = a.1 - b.get_pos().1 as f32;
    (x_dif.abs() + z_dif.abs()) as i32
}
