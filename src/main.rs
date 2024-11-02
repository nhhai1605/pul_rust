use pul_rust::my_libs::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            FpsPlugin,
            CameraPlugin,
            TablePlugin
        ))
        .run();
}
