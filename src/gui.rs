use bevy::{
    app::{Plugin, Update},
    // color::Color,
    prelude::{Commands, Entity, Query, ResMut},
};
use bevy_egui::{
    egui::{self},
    EguiContexts, EguiPlugin,
};

use crate::{chunk::ChunkId, world::World};

pub fn update_ui(
    mut world: ResMut<World>,
    mut contexts: EguiContexts,
    mut commands: Commands,
    mut chunks: Query<(Entity, &ChunkId)>,
) {
    let mut render_distance = world.render_distance;
    let mut HEIGHT_OFFSET = world.terrain_settings.NOISE_OFFSET;
    let mut HEIGHT_INTENSITY = world.terrain_settings.HEIGHT_INTENSITY;
    let mut NOISE_SCALE = world.terrain_settings.NOISE_SCALE;
    let mut NOISE_OFFSET = world.terrain_settings.NOISE_OFFSET;
    let mut NOISE_OCTAVES = world.terrain_settings.NOISE_OCTAVES;
    let mut NOISE_PERSISTENCE = world.terrain_settings.NOISE_PERSISTENCE;
    egui::Window::new("World").show(contexts.ctx_mut(), |ui| {
        ui.label("Settings");
        // render distance
        let old_render_distance = render_distance;
        ui.add(egui::Slider::new(&mut render_distance, 1..=50).text("render_distance"));
        if render_distance != old_render_distance {
            // this.set_size(height, width, depth);
            world.render_distance = render_distance;
        }

        let old_NOISE_OCTAVES = NOISE_OCTAVES;
        ui.add(egui::Slider::new(&mut NOISE_OCTAVES, 0..=20).text("NOISE_OCTAVES"));
        if NOISE_OCTAVES != old_NOISE_OCTAVES {
            world.terrain_settings.NOISE_OCTAVES = NOISE_OCTAVES;
            world.reset(&mut commands, &mut chunks);
        }

        let old_HEIGHT_INTENSITY = HEIGHT_INTENSITY;
        ui.add(egui::Slider::new(&mut HEIGHT_INTENSITY, 0.0..=256.0).text("HEIGHT_INTENSITY"));
        if HEIGHT_INTENSITY != old_HEIGHT_INTENSITY {
            world.terrain_settings.HEIGHT_INTENSITY = HEIGHT_INTENSITY;
            world.reset(&mut commands, &mut chunks);
        }
    });
}

pub struct GuiPlugin;
impl Plugin for GuiPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(EguiPlugin).add_systems(Update, update_ui);
    }
}
