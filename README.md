# Bevy Diagnostics Egui

This crate provides an easy way to display diagnostics information in a Bevy application using `egui`. It integrates seamlessly with Bevy's existing diagnostics infrastructure, allowing you to visualize performance metrics and other diagnostics data in a simple interface.

## Dependencies

- [bevy](https://github.com/bevyengine/bevy)
- [egui](https://github.com/emilk/egui)
- [bevy_egui](https://github.com/mvlabat/bevy_egui)

## Getting Started

To use `bevy_diagnostics_egui` in your project, add it as a dependency in your `Cargo.toml`:

```toml
[dependencies]
bevy = "0.13.2"
bevy_egui = "0.27.0"
egui = "0.27.2"
bevy_diagnostics_egui = { git = "https://github.com/insprac/bevy-diagnostics-egui.git" }
```

## Example Usage

Here's a simple example that sets up a Bevy application with default plugins, the `EguiPlugin`, and several diagnostics plugins, including the `DiagnosticsEguiPlugin`:

```rust
use bevy::prelude::*;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, SystemInformationDiagnosticsPlugin};
use bevy_egui::EguiPlugin;
use bevy_diagnostics_egui::DiagnosticsEguiPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .add_plugins(FrameTimeDiagnosticsPlugin) // Optionally add frame time diagnostics
        .add_plugins(SystemInformationDiagnosticsPlugin) // Optionally add system info diagnostics
        .add_plugins(DiagnosticsEguiPlugin)
        .run();
}
```
