use bevy::{
    color::palettes::css::WHITE,
    pbr::wireframe::WireframeConfig,
    prelude::*,
    render::{
        render_resource::WgpuFeatures,
        settings::{RenderCreation, WgpuSettings},
        RenderPlugin,
    },
};
use camera::RotatingCamera;
use custom_mesh::gen_visible_faces;
use input::input_handler;
use iyes_perf_ui::{entries::PerfUiBundle, PerfUiPlugin};

mod camera;
mod custom_mesh;
mod gui;
mod input;

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

    // let scale = 1.0;
    // let gap = 1.0;
    // let cube_size = scale + gap;
    // let cube_size = scale;
    let grid_size = 100;
    let mut cubes: Vec<Vec<Vec<(i32, i32, i32)>>> = vec![];
    for y in 0..grid_size {
        let mut t = vec![];
        for z in 0..grid_size {
            let mut m = vec![];
            for x in 0..grid_size {
                m.push((x, y, z));
            }
            t.push(m);
        }
        cubes.push(t);
    }
    let chunk_mesh = gen_visible_faces(&cubes);

    commands.spawn(PbrBundle {
        mesh: meshes.add(chunk_mesh),
        material: materials.add(StandardMaterial {
            base_color_texture: Some(custom_texture_handle.clone()),
            // base_color: Color::WHITE,
            base_color: Color::srgba(1.0, 1.0, 1.0, 1.0),
            // alpha_mode: AlphaMode::Mask(0.5),
            ..default()
        }),
        ..default()
    });

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
