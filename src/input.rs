// // Text to describe the controls.
// commands.spawn(
//     TextBundle::from_section(
//         "Controls:\nSpace: Change UVs\nX/Y/Z: Rotate\nR: Reset orientation",
//         TextStyle::default(),
//     )
//     .with_style(Style {
//         position_type: PositionType::Absolute,
//         top: Val::Px(12.0),
//         left: Val::Px(12.0),
//         ..default()
//     }),
// );

use bevy::{pbr::wireframe::WireframeConfig, prelude::*, render::mesh::VertexAttributeValues};

// Define a "marker" component to mark the custom mesh. Marker components are often used in Bevy for filtering entities in queries with `With`, they're usually not queried directly since they don't
// contain information within them.

// System to receive input from the user,
// check out examples/input/ for more examples about user input.
pub fn input_handler(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut wireframe: ResMut<WireframeConfig>,
    // mesh_query: Query<&Handle<Mesh>, With<CustomUV>>,
    // mut meshes: ResMut<Assets<Mesh>>,
    // mut query: Query<&mut Transform, With<CustomUV>>,
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
            // let mat = materials.get_mut(handle).unwrap();
            // mat.albedo_texture = Some(AssetPath::new("textures/uv_test.png").into());
        }
        // let mat_handle = material_query.get_many().expect("Query not successful");
        // let mesh = materials.get_mut(mat_handle).unwrap();
    }
}

// Function that changes the UV mapping of the mesh, to apply the other texture.
// fn toggle_texture(mesh_to_change: &mut Mesh) {
//     // Get a mutable reference to the values of the UV attribute, so we can iterate over it.
//     let uv_attribute = mesh_to_change.attribute_mut(Mesh::ATTRIBUTE_UV_0).unwrap();
//     // The format of the UV coordinates should be Float32x2.
//     let VertexAttributeValues::Float32x2(uv_attribute) = uv_attribute else {
//         panic!("Unexpected vertex format, expected Float32x2.");
//     };

//     // Iterate over the UV coordinates, and change them as we want.
//     for uv_coord in uv_attribute.iter_mut() {
//         // If the UV coordinate points to the upper, "dirt+grass" part of the texture...
//         if (uv_coord[1] + 0.5) < 1.0 {
//             // ... point to the equivalent lower, "sand+water" part instead,
//             uv_coord[1] += 0.5;
//         } else {
//             // else, point back to the upper, "dirt+grass" part.
//             uv_coord[1] -= 0.5;
//         }
//     }
// }
