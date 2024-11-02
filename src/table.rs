use crate::my_libs::*;

pub struct TablePlugin;

impl Plugin for TablePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((Wireframe2dPlugin,));
        app.add_systems(Startup, setup);
        app.add_systems(Update, toggle_wireframe);
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

    commands.spawn(
        TextBundle::from_section("Press space to toggle wireframes", TextStyle {
            font_size: 16.0,
            color: Color::WHITE,
            ..default()
        })
            .with_style(Style {
                position_type: PositionType::Absolute,
                left: Val::Percent(1.),
                top: Val::Percent(1.),
                bottom: Val::Auto,
                right: Val::Auto,
                ..default()
            }),
    );
}

fn toggle_wireframe(
    mut wireframe_config: ResMut<Wireframe2dConfig>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        wireframe_config.global = !wireframe_config.global;
    }
}
