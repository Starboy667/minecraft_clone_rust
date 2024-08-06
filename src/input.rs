use bevy::{pbr::wireframe::WireframeConfig, prelude::*};

pub fn input_handler(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut wireframe: ResMut<WireframeConfig>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    material_query: Query<&Handle<StandardMaterial>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        wireframe.global = !wireframe.global;
        for handle in material_query.iter() {
            let mat = materials.get_mut(handle).unwrap();
            if mat.base_color == Color::srgba(1.0, 1.0, 1.0, 0.0) {
                mat.base_color = Color::srgba(1.0, 1.0, 1.0, 1.0);
                mat.alpha_mode = AlphaMode::Opaque;
            } else {
                mat.base_color = Color::srgba(1.0, 1.0, 1.0, 0.0);
                mat.alpha_mode = AlphaMode::Mask(0.5);
            }
        }
    }
}
