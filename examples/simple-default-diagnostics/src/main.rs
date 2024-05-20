use bevy::diagnostic::{EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin, SystemInformationDiagnosticsPlugin};
use bevy::prelude::*;
use bevy_diagnostics_egui::DiagnosticsEguiPlugin;
use bevy_egui::EguiPlugin;

fn main() {
    App::new()
        // Bevy's default plugins
        .add_plugins(DefaultPlugins)
        // The EguiPlugin is necessary for the DiagnosticsEguiPlugin
        .add_plugins(EguiPlugin)
        // Add optional diagnostics, you can add custom diagnostics as well
        .add_plugins(FrameTimeDiagnosticsPlugin)
        .add_plugins(SystemInformationDiagnosticsPlugin)
        .add_plugins(EntityCountDiagnosticsPlugin)
        // Add the egui diagnostics window
        .add_plugins(DiagnosticsEguiPlugin)
        .run();
}
