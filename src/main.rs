// Load and use this module on debug
#[cfg(debug_assertions)]
mod debug_plugin;

use bevy::{prelude::*, window::WindowResolution};
use config::{GAME_HEIGHT, GAME_WIDTH, WINDOW_TITLE};

mod asset_loading;
mod camera;
mod config;
mod flow_control;

fn main() {
    let mut app = App::new();

    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: WindowResolution::new(GAME_WIDTH, GAME_HEIGHT),
                    title: WINDOW_TITLE.to_string(),
                    ..default()
                }),
                ..default()
            })
            .set(ImagePlugin::default_nearest()),
    )
    .insert_resource(Msaa::Off);

    // Add this plugins and system on debug
    #[cfg(debug_assertions)]
    app.add_plugins(debug_plugin::Plug);

    app.add_plugins((camera::Plug, flow_control::Plug, asset_loading::Plug));

    app.run();
}
