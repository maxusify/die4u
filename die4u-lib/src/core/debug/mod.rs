use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*, window::PrimaryWindow};
use bevy_egui::{egui, EguiContext, EguiPlugin};
use bevy_inspector_egui::{
    bevy_inspector::{ui_for_assets, ui_for_resources, ui_for_world_entities},
    DefaultInspectorConfigPlugin,
};

pub struct GameDebugPlugin;

impl Plugin for GameDebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(EguiPlugin);
        app.add_plugin(DefaultInspectorConfigPlugin);
        app.add_plugin(FrameTimeDiagnosticsPlugin::default());
        app.add_system(debug_inspector_ui);
    }
}

/// UI for debugging
fn debug_inspector_ui(world: &mut World, mut disabled: Local<bool>) {
    let space_pressed = world
        .resource::<Input<KeyCode>>()
        .just_pressed(KeyCode::Space);

    if space_pressed {
        *disabled = !*disabled;
    }

    if *disabled {
        return;
    }

    let mut egui_context = world
        .query_filtered::<&mut EguiContext, With<PrimaryWindow>>()
        .single(world)
        .clone();

    egui::Window::new("Debug Overlay").show(egui_context.get_mut(), |ui| {
        egui::ScrollArea::vertical().show(ui, |ui| {
            egui::CollapsingHeader::new("Materials").show(ui, |ui| {
                ui_for_assets::<StandardMaterial>(world, ui);
            });

            egui::CollapsingHeader::new("Entities").show(ui, |ui| {
                ui_for_world_entities(world, ui);
            });

            egui::CollapsingHeader::new("Resources").show(ui, |ui| {
                ui_for_resources(world, ui);
            });

            ui.separator();
            ui.label("Press SPACE to toggle");
        });
    });
}
