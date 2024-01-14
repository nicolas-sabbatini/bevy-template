// Load and use this module on debug
#[cfg(debug_assertions)]
mod debug_plugin;

use asset_loading::AssetLoadingPlugin;
use bevy::{prelude::*, window::WindowResolution};
use camera::CameraPlugin;
use config::*;
use flow_control::FlowControlPlugin;

mod asset_loading;
mod camera;
mod config;
mod constants;
mod flow_control;

fn main() {
    let mut app = App::new();

    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: WindowResolution::new(WINDOW_WIDTH * 2.0, WINDOW_HEIGHT * 2.0),
                    title: WINDOW_TITLE.to_string(),
                    fit_canvas_to_parent: true,
                    ..default()
                }),
                ..default()
            })
            .set(ImagePlugin::default_nearest()),
    )
    .insert_resource(Msaa::Off);

    // // Add this plugins and system on debug
    #[cfg(debug_assertions)]
    app.add_plugins(debug_plugin::DebugPlugin);

    app.add_plugins((CameraPlugin, FlowControlPlugin, AssetLoadingPlugin));

    app.run();
}
