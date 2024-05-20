use bevy::diagnostic::DiagnosticsStore;
use bevy::prelude::*;
use bevy_egui::EguiContexts;

pub struct DiagnosticsEguiPlugin;

impl Plugin for DiagnosticsEguiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, draw_diagnostic_ui);
    }
}

fn draw_diagnostic_ui(diagnostics: Res<DiagnosticsStore>, mut contexts: EguiContexts) {
    egui::Window::new("Diagnostics").show(contexts.ctx_mut(), |ui| {
        for diagnostic in diagnostics.iter() {
            if !diagnostic.is_enabled {
                continue;
            }
            let Some(value) = diagnostic.smoothed() else {
                continue;
            };
            ui.label(format!(
                "{}: {:.2}{}",
                diagnostic.path(),
                value,
                diagnostic.suffix,
            ));
        }
    });
}
