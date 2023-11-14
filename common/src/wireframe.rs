use bevy::{prelude::*,
    pbr::wireframe::{WireframeColor, WireframeConfig}, // можно добавить Wirefarame и NoWireframe к отдельным объектам для выбора рендера
};

#[derive(Component)]
pub struct WireframeButton;

impl WireframeButton{
    pub fn update_colors(
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
}