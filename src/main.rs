use bevy::prelude::*;

fn main(){
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window{
                title: "Atherforge".to_string(),
                resolution: (1280, 720).into(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, player_movement)
        .run();
}

#[derive(Component)]
struct Player;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(20.0, 20.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.2, 0.5, 0.2))),
    ));
    
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.7, 0.2, 0.2))),
    ));
    
    commands.spawn((
        DirectionalLight {
            shadow_maps_enabled: true,
            illuminance: 10_000.0,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(
            EulerRot::XYZ,
            -1.0,
            -0.5,
            0.0,
        )),
    ));
    
    commands.spawn((
        Camera3d::default(),
        Player,
        Transform::from_xyz(4.0, 4.0, 6.0)
            .looking_at(Vec3::ZERO, Vec3::Y),  
    ));

}

fn player_movement(
    kb_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut player_query: Query<&mut Transform, With<Player>>
){
    let Ok(mut transform) = player_query.single_mut() else {
        return;
    };

    let mut direction = Vec3::ZERO;

    if kb_input.pressed(KeyCode::KeyW) {
        direction.z -= 1.0;
    }
    if kb_input.pressed(KeyCode::KeyA) {
        direction.x -= 1.0;
    }
    if kb_input.pressed(KeyCode::KeyS) {
        direction.z += 1.0;
    }
    if kb_input.pressed(KeyCode::KeyD) {
        direction.x += 1.0;
    }

    let movement_speed = 5.0;

    transform.translation +=
        direction.normalize_or_zero() * movement_speed * time.delta_secs();
    
}