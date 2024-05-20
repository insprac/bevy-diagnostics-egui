use bevy::diagnostic::{Diagnostic, DiagnosticsStore};
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
            let Some(value) = get_diagnostic_value(diagnostic) else {
                continue;
            };
            ui.label(format!(
                "{}: {}{}",
                diagnostic.path(),
                format_value(value),
                diagnostic.suffix,
            ));
        }
    });
}

fn get_diagnostic_value(diagnostic: &Diagnostic) -> Option<f64> {
    if !diagnostic.is_enabled {
        return None;
    }
    diagnostic.smoothed()
}

fn format_value(value: f64) -> String {
    format!("{:.2}", value)
        .trim_end_matches('0')
        .trim_end_matches('.')
        .to_owned()
}
