use bevy::{
    color::palettes::css::WHITE,
    pbr::wireframe::WireframeConfig,
    prelude::*,
    render::{
        render_resource::WgpuFeatures,
        settings::{RenderCreation, WgpuSettings},
        view::NoFrustumCulling,
        RenderPlugin,
    },
};
use camera::RotatingCamera;
use custom_mesh::create_cube_mesh;
use input::input_handler;
use iyes_perf_ui::{entries::PerfUiBundle, PerfUiPlugin};
use render::{CustomMaterialPlugin, InstanceData, InstanceMaterialData};

mod camera;
mod custom_mesh;
mod gui;
mod input;
mod render;

#[derive(Component)]
pub struct CustomUV;

fn main() {
    let mut wgpu_settings = WgpuSettings::default();
    wgpu_settings.features |= WgpuFeatures::POLYGON_MODE_LINE;
    App::new()
        .add_plugins((DefaultPlugins.set(RenderPlugin {
            render_creation: RenderCreation::Automatic(WgpuSettings {
                features: WgpuFeatures::POLYGON_MODE_LINE,
                ..default()
            }),
            ..default()
        }),))
        // .add_plugins(CustomMaterialPlugin)
        .add_plugins(bevy::pbr::wireframe::WireframePlugin)
        .add_plugins(camera::RotatingCameraPlugin)
        // GUI
        .add_plugins(gui::GuiPlugin)
        // FPS
        .add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
        .add_plugins(bevy::diagnostic::EntityCountDiagnosticsPlugin)
        .add_plugins(bevy::diagnostic::SystemInformationDiagnosticsPlugin)
        .add_plugins(PerfUiPlugin)
        .insert_resource(WireframeConfig {
            global: false,
            default_color: WHITE.into(),
        })
        .add_systems(Startup, setup)
        .add_systems(Update, input_handler)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    // Import the custom texture.
    let custom_texture_handle: Handle<Image> = asset_server.load("array_texture.png");
    // Create and save a handle to the mesh.
    let cube_mesh_handle: Handle<Mesh> = meshes.add(create_cube_mesh());

    // Render the mesh with the custom texture using a PbrBundle, add the marker.
    let scale = 1.0;
    let grid_size = 20;
    let gap = 1.0;
    // let cube_size = scale + gap;
    let cube_size = scale;
    for x in 0..grid_size {
        for y in 0..grid_size {
            for z in 0..grid_size {
                commands.spawn(PbrBundle {
                    mesh: cube_mesh_handle.clone(),
                    material: materials.add(StandardMaterial {
                        base_color_texture: Some(custom_texture_handle.clone()),
                        // base_color: Color::WHITE,
                        base_color: Color::srgba(1.0, 1.0, 1.0, 1.0),
                        // alpha_mode: AlphaMode::Mask(0.5),
                        ..default()
                    }),
                    transform: Transform::from_xyz(
                        (x as f32 * cube_size) - (grid_size as f32 * cube_size / 2.0),
                        (y as f32 * cube_size) - (grid_size as f32 * cube_size / 2.0),
                        (z as f32 * cube_size) - (grid_size as f32 * cube_size / 2.0),
                    ),
                    ..default()
                });
            }
        }
    }
    // commands.spawn(PbrBundle {
    //     mesh: cube_mesh_handle,
    //     material: materials.add(StandardMaterial {
    //         base_color_texture: Some(custom_texture_handle),
    //         base_color: Color::srgba(1.0, 1.0, 1.0, 0.0),
    //         ..default()
    //     }),
    //     ..default()
    // });
    // .insert(InstanceMaterialData(
    //     (1..=grid_size)
    //         .flat_map(|x| (1..=grid_size).map(move |y| (x as f32, y as f32)))
    //         .map(|(x, y)| InstanceData {
    //             position: Vec3::new(
    //                 (x * cube_size) - (grid_size as f32 * cube_size / 2.0),
    //                 (y * cube_size) - (grid_size as f32 * cube_size / 2.0),
    //                 0.0,
    //             ),
    //             scale: scale,
    //             // color: Color::WHITE.with_alpha(0.0).to_linear().to_f32_array(),
    //             color: LinearRgba::new(1.0, 1.0, 1.0, 0.0).to_f32_array(),
    //         })
    //         .collect::<Vec<_>>(),
    // ))
    // .insert(NoFrustumCulling);

    // Transform for the camera and lighting, looking at (0,0,0) (the position of the mesh).
    let camera_and_light_transform =
        Transform::from_xyz(1.8, 1.8, 1.8).looking_at(Vec3::ZERO, Vec3::Y);

    // Camera in 3D space.
    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_xyz(1.8, 5.0, 1.8).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        })
        .insert(RotatingCamera::default());

    // Light up the scene.
    commands.spawn(PointLightBundle {
        transform: camera_and_light_transform,
        ..default()
    });
    commands.spawn(PerfUiBundle::default());
}
