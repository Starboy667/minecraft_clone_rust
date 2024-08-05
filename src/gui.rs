use bevy::{
    app::{Plugin, Update},
    color::Color,
    prelude::ResMut,
};
use bevy_egui::{
    egui::{self, Color32},
    EguiContexts, EguiPlugin,
};

pub fn update_ui(
    // mut this: ResMut<Sims>,
    mut contexts: EguiContexts,
) {
    // let mut bounds = this.bounds;
    egui::Window::new("Simulation").show(contexts.ctx_mut(), |ui| {
        ui.label("Settings");
        // // BOUNDS
        // let old_bounds = bounds;
        // ui.add(egui::Slider::new(&mut bounds, 30..=300).text("bounds"));
        // if bounds != old_bounds {
        //     this.set_size(bounds);
        // }
        // // SPEED
        // let mut speed = this.update_timer.duration().as_secs_f32();
        // ui.add(egui::Slider::new(&mut speed, 0.0..=0.5).text("speed"));
        // if speed != this.update_timer.duration().as_secs_f32() {
        //     this.update_timer
        //         .set_duration(std::time::Duration::from_secs_f32(speed));
        // }
        // // RESET
        // if ui.button("Reset").clicked() {
        //     this.reset();
        // }
        // ui.label("Rule");
        // ui.horizontal_wrapped(|ui| {
        //     for i in 0..this.rule_preset.len() {
        //         if ui.button(&this.rule_preset[i].name).clicked() {
        //             this.load_rule_preset(i);
        //         }
        //     }
        // });

        // COLOR
        // ui.label("Color mode");
        // ui.checkbox(&mut this.glow, "Glow");
        // ui.horizontal(|ui| {
        //     ui.radio_value(&mut this.color_handler, ColorHandler::Rgb, "RGB");
        //     ui.radio_value(
        //         &mut this.color_handler,
        //         ColorHandler::ColorPalette,
        //         "ColorPalette",
        //     );
        //     ui.radio_value(
        //         &mut this.color_handler,
        //         ColorHandler::StateShading,
        //         "StateShading",
        //     );
        //     ui.radio_value(
        //         &mut this.color_handler,
        //         ColorHandler::NeighborhoodDensity,
        //         "NeighborhoodDensity",
        //     );
        // });
        // if this.color_handler == ColorHandler::ColorPalette
        //     || this.color_handler == ColorHandler::StateShading
        // {
        //     ui.label("Color palette");
        //     color_picker(ui, &mut this.color_palette[0]);
        //     color_picker(ui, &mut this.color_palette[1]);
        // }
    });
}

pub struct GuiPlugin;
impl Plugin for GuiPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(EguiPlugin).add_systems(Update, update_ui);
    }
}
