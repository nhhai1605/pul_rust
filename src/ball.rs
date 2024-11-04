use crate::my_libs::*;
pub struct BallPlugin;

#[derive(Component)]
pub struct Ball;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Update, movement);
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    // commands.spawn((
    //     SpriteBundle {
    //         texture: asset_server.load("ball_0.png"),
    //         transform: Transform::from_xyz(0., 0., 0.).with_scale(Vec3::splat(0.02)),
    //         ..default()
    //     },
    //     RigidBody::Dynamic,
    //     Collider::circle(5.0),
    //     // Rotate,
    //     PIXEL_PERFECT_LAYERS
    // ));
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Circle { radius: 5.0 })),
            material: materials.add(Color::srgb(0.2, 0.7, 0.9)),
            transform: Transform::from_xyz(0., 0., 1.),
            ..default()
        },
        RigidBody::Dynamic,
        Collider::circle(5.0),
        Ball,
    ));
}

fn movement(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut balls: Query<&mut LinearVelocity, With<Ball>>,
) {
    // Precision is adjusted so that the example works with
    // both the `f32` and `f64` features. Otherwise you don't need this.
    let delta_time = time.delta().as_secs_f32();
    println!("delta_time: {}", delta_time);
    println!("linear_velocity: {:?}", balls);
    for mut linear_velocity in &mut balls {
        if keyboard_input.any_pressed([KeyCode::KeyW, KeyCode::ArrowUp]) {
            // Use a higher acceleration for upwards movement to overcome gravity
            linear_velocity.y += 100.0 * delta_time;
        }
        if keyboard_input.any_pressed([KeyCode::KeyS, KeyCode::ArrowDown]) {
            linear_velocity.y -= 100.0 * delta_time;
        }
        if keyboard_input.any_pressed([KeyCode::KeyA, KeyCode::ArrowLeft]) {
            linear_velocity.x -= 100.0 * delta_time;
        }
        if keyboard_input.any_pressed([KeyCode::KeyD, KeyCode::ArrowRight]) {
            linear_velocity.x += 100.0 * delta_time;
        }
    }
}
