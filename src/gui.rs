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
    let mut NOISE_OCTAVES = world.terrain_settings.NOISE_OCTAVES;
    let mut NOISE_PERSISTENCE = world.terrain_settings.NOISE_PERSISTENCE;
    let mut a = world.terrain_settings.a;
    let mut b = world.terrain_settings.b;
    let mut c = world.terrain_settings.c;
    let mut d = world.terrain_settings.d;
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

        let old_NOISE_PERSISTENCE = NOISE_PERSISTENCE;
        ui.add(egui::Slider::new(&mut NOISE_PERSISTENCE, 0.0..=5.0).text("NOISE_PERSISTENCE"));
        if NOISE_PERSISTENCE != old_NOISE_PERSISTENCE {
            world.terrain_settings.NOISE_PERSISTENCE = NOISE_PERSISTENCE;
            world.reset(&mut commands, &mut chunks);
        }

        let old_a = a;
        ui.add(egui::Slider::new(&mut a.0, -1.0..=2.0).text("a.0"));
        ui.add(egui::Slider::new(&mut a.1, 0.0..=200.0).text("a.1"));
        if a != old_a {
            world.terrain_settings.a = a;
            world.reset(&mut commands, &mut chunks);
        }
        let old_b = b;
        ui.add(egui::Slider::new(&mut b.0, -1.0..=2.0).text("b.0"));
        ui.add(egui::Slider::new(&mut b.1, 0.0..=200.0).text("b.1"));
        if b != old_b {
            world.terrain_settings.b = b;
            world.reset(&mut commands, &mut chunks);
        }
        let old_c = c;
        ui.add(egui::Slider::new(&mut c.0, -1.0..=2.0).text("c.0"));
        ui.add(egui::Slider::new(&mut c.1, 0.0..=200.0).text("c.1"));
        if c != old_c {
            world.terrain_settings.c = c;
            world.reset(&mut commands, &mut chunks);
        }
        let old_d = d;
        ui.add(egui::Slider::new(&mut d.0, -1.0..=2.0).text("d.0"));
        ui.add(egui::Slider::new(&mut d.1, 0.0..=200.0).text("d.1"));
        if d != old_d {
            world.terrain_settings.d = d;
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
