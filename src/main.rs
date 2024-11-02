use bevy::{
    prelude::*,
    sprite::*,
};

use pul_rust::{
    fps::FpsPlugin,
    camera::CameraPlugin
};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            Wireframe2dPlugin,
            FpsPlugin,
            CameraPlugin
        ))
        .run();
}
