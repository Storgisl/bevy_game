use bevy::{prelude::*,
    input::mouse::MouseMotion,
    pbr::wireframe::{NoWireframe, Wireframe, WireframeColor, WireframeConfig, WireframePlugin},
    render::{
        render_resource::WgpuFeatures,
        settings::{RenderCreation, WgpuSettings},
        RenderPlugin,
    },
};
#[derive(Component)]
struct WireframeButton;

#[derive(Component)]
struct Camera {
    speed: f32,
}
fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(RenderPlugin {
                render_creation: RenderCreation::Automatic(WgpuSettings {
                    // WARN this is a native only feature. It will not work with webgl or webgpu
                    features: WgpuFeatures::POLYGON_MODE_LINE,
                    ..default()
                }),
            }),
            // You need to add this plugin to enable wireframe rendering
            WireframePlugin,
        ))
        // Wireframes can be configured with this resource. This can be changed at runtime.
        .insert_resource(WireframeConfig {
            // The global wireframe config enables drawing of wireframes on every mesh,
            // except those with `NoWireframe`. Meshes with `Wireframe` will always have a wireframe,
            // regardless of the global configuration.
            global: true,
            // Controls the default color of all wireframes. Used as the default color for global wireframes.
            // Can be changed per mesh using the `WireframeColor` component.
            default_color: Color::WHITE,
        })
        .add_systems(Startup, setup)
        .add_systems(Update, (camera_movement, camera_rotation, update_colors))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // wireframe 
    commands.spawn(ButtonBundle::default())
    .insert(WireframeButton);
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
    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 2.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    }).insert(Camera { speed: 5.0 });
    // text
    commands.spawn(
        TextBundle::from_section("", TextStyle::default()).with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        }),
    );
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

fn update_colors(
    keyboard_input: Res<Input<KeyCode>>,
    mut config: ResMut<WireframeConfig>,
    mut wireframe_colors: Query<&mut WireframeColor>,
    mut text: Query<&mut Text>,
) {
    text.single_mut().sections[0].value = format!(
        "
Controls
---------------
Z - Toggle global
X - Change global color
C - Change color of the green cube wireframe

WireframeConfig
-------------
Global: {}
Color: {:?}
",
        config.global, config.default_color,
    );

    // Toggle showing a wireframe on all meshes
    if keyboard_input.just_pressed(KeyCode::Z) {
        config.global = !config.global;
    }

    // Toggle the global wireframe color
    if keyboard_input.just_pressed(KeyCode::X) {
        config.default_color = if config.default_color == Color::WHITE {
            Color::PINK
        } else {
            Color::WHITE
        };
    }

    // Toggle the color of a wireframe using WireframeColor and not the global color
    if keyboard_input.just_pressed(KeyCode::C) {
        for mut color in &mut wireframe_colors {
            color.color = if color.color == Color::GREEN {
                Color::RED
            } else {
                Color::GREEN
            };
        }
    }
}