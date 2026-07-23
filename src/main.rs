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
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut player_query: Query<&mut Transform, With<Player>>,
) {
    let Ok(mut transform) = player_query.single_mut() else {
        return;
    };

    let mut input_direction = Vec3::ZERO;

    if keyboard_input.pressed(KeyCode::KeyW) {
        input_direction.z -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::KeyS) {
        input_direction.z += 1.0;
    }

    if keyboard_input.pressed(KeyCode::KeyA) {
        input_direction.x -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::KeyD) {
        input_direction.x += 1.0;
    }

    let forward = transform.rotation * Vec3::NEG_Z;
    let right = transform.rotation * Vec3::X;

    let forward = Vec3::new(forward.x, 0.0, forward.z).normalize_or_zero();
    let right = Vec3::new(right.x, 0.0, right.z).normalize_or_zero();

    let movement_direction =
        forward * -input_direction.z + right * input_direction.x;

    let movement_speed = 5.0;

    transform.translation +=
        movement_direction.normalize_or_zero()
            * movement_speed
            * time.delta_secs();
}