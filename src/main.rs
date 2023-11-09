use bevy::prelude::*;
use bevy::input::mouse::MouseMotion;
use bevy::pbr::wireframe::WireframePlugin;

#[derive(Component)]
struct Camera {
    speed: f32,
}
fn main() {
    App::new()
        .add_plugins((DefaultPlugins, WireframePlugin))
        .add_systems(Startup, setup)
        .add_systems(Update, (camera_movement, camera_rotation))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // circular base
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Circle::new(4.0).into()),
        material: materials.add(Color::WHITE.into()),
        transform: Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
        ..default()
    });
    // cube
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb_u8(124, 100, 255).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    });
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 2.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    }).insert(Camera { speed: 5.0 });
}

fn camera_movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Camera, &mut Transform)>
) {
    for (camera, mut transform) in query.iter_mut() {
        let mut translation = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::W) {
            translation += -transform.local_z().normalize();
        }
        if keyboard_input.pressed(KeyCode::S) {
            translation += transform.local_z().normalize();
        }
        if keyboard_input.pressed(KeyCode::A) {
            translation += -transform.local_x().normalize();
        }
        if keyboard_input.pressed(KeyCode::D) {
            translation += transform.local_x().normalize();
        }
        transform.translation += translation * camera.speed * time.delta_seconds();

    }
}

fn camera_rotation(
    time: Res<Time>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut query: Query<(&Camera, &mut Transform)>
) {
    for (camera, mut transform) in query.iter_mut() {
        let translation = Vec3::ZERO;
        let sensitivity = 0.002;

        transform.translation += translation * camera.speed * time.delta_seconds();

        for event in mouse_motion_events.read() {
            let mouse_delta = event.delta;

            let yaw = -mouse_delta.x * sensitivity;
            let pitch = -mouse_delta.y * sensitivity;

            let yaw_rotation = Quat::from_rotation_y(yaw);
            let pitch_rotation = Quat::from_rotation_x(pitch);

            transform.rotation = yaw_rotation * transform.rotation;
            transform.rotation = transform.rotation * pitch_rotation;
        }
    }
}