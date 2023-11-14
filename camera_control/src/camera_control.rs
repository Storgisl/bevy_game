use bevy::{prelude::*,
    input::mouse::MouseMotion,
};

#[derive(Component)]
pub struct CameraControl {
    pub speed: f32,
}
impl CameraControl{
    pub fn camera_movement(
        time: Res<Time>,
        keyboard_input: Res<Input<KeyCode>>,
        mut query: Query<(&CameraControl, &mut Transform)>
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

    pub fn camera_rotation(
        time: Res<Time>,
        mut mouse_motion_events: EventReader<MouseMotion>,
        mut query: Query<(&CameraControl, &mut Transform)>
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
}