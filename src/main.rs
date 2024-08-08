use bevy::{
    color::palettes::css::WHITE,
    pbr::wireframe::WireframeConfig,
    prelude::{Sphere, *},
    render::{
        render_resource::WgpuFeatures,
        settings::{RenderCreation, WgpuSettings},
        RenderPlugin,
    },
};
use bevy_panorbit_camera::PanOrbitCamera;
use camera::RotatingCamera;
use chunk::Chunk;
use custom_mesh::gen_visible_faces;
use input::input_handler;
use iyes_perf_ui::{entries::PerfUiBundle, PerfUiPlugin};
use player::Player;
use world::RENDER_DISTANCE;

mod camera;
mod chunk;
mod custom_mesh;
mod gui;
mod input;
mod player;
mod world;

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
        .add_plugins(world::WorldPlugin)
        // .add_plugins(camera::RotatingCameraPlugin)
        .add_plugins(bevy_panorbit_camera::PanOrbitCameraPlugin)
        // FPS
        .add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
        .add_plugins(bevy::diagnostic::EntityCountDiagnosticsPlugin)
        .add_plugins(bevy::diagnostic::SystemInformationDiagnosticsPlugin)
        .add_plugins(PerfUiPlugin)
        // WIREFRAME
        .add_plugins(bevy::pbr::wireframe::WireframePlugin)
        .insert_resource(WireframeConfig {
            global: false,
            default_color: WHITE.into(),
        })
        // GUI
        .add_plugins(gui::GuiPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, input_handler)
        // oui
        .add_systems(Update, move_player)
        .run();
}

pub fn move_player(mut cameras: Query<(&mut RotatingCamera, &mut Transform), With<Player>>) {
    for (mut camera, mut transform) in cameras.iter_mut() {
        let delta = 1.0f32;
        camera.rotation += delta * 0.10f32;
        let rotation = Quat::from_axis_angle(Vec3::Y, camera.rotation);
        transform.translation = Vec3::new(100.0, 75.0, 100.0) + (rotation * Vec3::Z * 40f32);
        transform.look_at(camera.center, Vec3::Y);
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    // Import the custom texture.
    let custom_texture_handle: Handle<Image> = asset_server.load("array_texture.png");

    for i in 0..RENDER_DISTANCE + 2 {
        for j in 0..RENDER_DISTANCE + 2 {
            commands
                .spawn(PbrBundle {
                    mesh: meshes.add(Cuboid {
                        ..Default::default()
                    }),
                    material: materials.add(StandardMaterial {
                        base_color: Color::WHITE,
                        base_color_texture: Some(custom_texture_handle.clone()),
                        ..default()
                    }),
                    transform: Transform::from_xyz(0.0, 0.0, 0.0),
                    ..default()
                })
                .insert(Chunk::default());
        }
    }
    // commands
    //     .spawn(PbrBundle {
    //         mesh: meshes.add(Cuboid {
    //             ..Default::default()
    //         }),
    //         material: materials.add(StandardMaterial {
    //             base_color_texture: Some(custom_texture_handle.clone()),
    //             // base_color: Color::WHITE,
    //             base_color: Color::srgba(1.0, 1.0, 1.0, 1.0),
    //             // alpha_mode: AlphaMode::Mask(0.5),
    //             ..default()
    //         }),
    //         transform: Transform::from_xyz(0.0, 0.0, 0.0),
    //         ..default()
    //     })
    //     .insert(Chunk::default());

    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Sphere {
                radius: 10.0,
                ..Default::default()
            }),
            material: materials.add(StandardMaterial {
                base_color: Color::srgb(1.0, 0.0, 1.0),
                ..default()
            }),
            transform: Transform::from_xyz(0.0, 100.0, 0.0),
            ..default()
        })
        .insert(Player::default())
        .insert(RotatingCamera::default());

    // Camera in 3D space.
    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_xyz(100.8, 5.0, -50.8).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        })
        // .insert(RotatingCamera::default());
        .insert(PanOrbitCamera::default());

    // Light up the scene.
    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_translation(Vec3::new(0., 256., 0.))
            .looking_at(Vec3::ZERO, Vec3::Y),
        directional_light: DirectionalLight {
            // shadows_enabled: true,
            ..Default::default()
        },
        ..Default::default()
    });
    commands.spawn(PerfUiBundle::default());
}
