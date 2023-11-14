use bevy::{prelude::*,
    core_pipeline::Skybox,
    pbr::wireframe::{ WireframeConfig, WireframePlugin}, // можно добавить Wirefarame и NoWireframe к отдельным объектам для выбора рендера
    render::{
        render_resource::WgpuFeatures,
        settings::{RenderCreation, WgpuSettings},
        RenderPlugin,
    },
};

use camera_control::camera_control::CameraControl;
use common::skybox::Cubemap;
use common::wireframe::WireframeButton;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(RenderPlugin {
                render_creation: RenderCreation::Automatic(WgpuSettings {
                    features: WgpuFeatures::POLYGON_MODE_LINE,
                    ..default()
                }),
            }),
            WireframePlugin,
        ))
        .insert_resource(WireframeConfig {
            global: true,
            default_color: Color::WHITE,
        })
        .add_systems(Startup, setup)
        .add_systems(Update, (CameraControl::camera_movement, CameraControl::camera_rotation, WireframeButton::update_colors, Cubemap::cycle_cubemap_asset, 
            Cubemap::skybox_initialization.after(Cubemap::cycle_cubemap_asset)))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>
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
    let skybox_handle = asset_server.load(Cubemap::CUBEMAPS[0].0);
    // camera
    commands.spawn((Camera3dBundle {
        transform: Transform::from_xyz(0.0, 2.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    },
    Skybox(skybox_handle.clone()),
    )).insert(CameraControl { speed: 5.0 });
    // text
    commands.spawn(
        TextBundle::from_section("", TextStyle::default()).with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        }),
    );
    commands.insert_resource(Cubemap {
        is_loaded: false,
        index: 0,
        image_handle: skybox_handle,
    });
}