use crate::my_libs::*;

pub struct TablePlugin;

impl Plugin for TablePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((Wireframe2dPlugin));
        app.add_systems(Startup, setup);
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(MaterialMesh2dBundle {
        mesh: Mesh2dHandle(meshes.add(Rectangle::new(TABLE_WIDTH, TABLE_HEIGHT))),
        material: materials.add(Color::srgb(21. / 255., 88. / 255., 67. / 255.)),
        transform: Transform::from_xyz(500., 0., 0.),
        ..default()
    });
}
