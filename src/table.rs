use crate::my_libs::*;
pub struct TablePlugin;

#[derive(Component)]
pub struct Table;

#[derive(Component)]
struct Rotate;

impl Plugin for TablePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Update, rotate);
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("table.png"),
            transform: Transform::from_xyz(0., 0., 0.).with_scale(Vec3::splat(0.02)),
            ..default()
        },
        RigidBody::Static,
        Collider::rectangle(TABLE_WIDTH, TABLE_HEIGHT),
        // Rotate,
        Table,
        PIXEL_PERFECT_LAYERS
    ));
}

fn rotate(time: Res<Time>, mut transforms: Query<&mut Transform, With<Rotate>>) {
    for mut transform in &mut transforms {
        let dt = time.delta_seconds();
        transform.rotate_z(dt);
    }
}
