use pul_rust::my_libs::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Pul".to_string(),
                    resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
                    ..default()
                }),
                ..default()
            }),
            HUDPlugin,
            CameraPlugin,
            TablePlugin
        ))
        .run();
}
