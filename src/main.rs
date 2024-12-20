use pul_rust::my_libs::*;
// For pixel rendering, I use this tutorial: https://bevyengine.org/examples-webgpu/2d-rendering/pixel-grid-snap/


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
            }).set(ImagePlugin::default_nearest()), //Need this for clear pixel rendering
            PhysicsPlugins::default().with_length_unit(100.0),
            Wireframe2dPlugin,
            HUDPlugin,
            MainCameraPlugin,
            TablePlugin,
            BallPlugin
        ))
        .insert_resource(Msaa::Off) //Need this for clear pixel rendering
        .run();
}
