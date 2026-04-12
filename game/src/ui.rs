use bevy::prelude::*;
use crate::player::Player;
use crate::map::PlayerSpawnPoint;

#[derive(Message, Clone)]
pub struct HeightChanged(pub f32);

#[derive(Component)]
pub struct HeightText;

pub fn setup_ui(mut commands: Commands) {
    commands.spawn((
        Text::new("Height: 0"),
        TextFont {
            font_size: 32.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(16.0),
            left: Val::Px(16.0),
            ..default()
        },
        HeightText,
    ));
}

pub fn detect_height_change(
    player_query: Query<&Transform, With<Player>>,
    spawn_point: Res<PlayerSpawnPoint>,
    mut last_height: Local<f32>,
    mut writer: MessageWriter<HeightChanged>,
) {
    let Ok(transform) = player_query.single() else {
        return;
    };

    let current_height = (((transform.translation.y) / 10.0)-10.0).round();

    if (current_height - *last_height).abs() >= 1.0 {
        *last_height = current_height;
        writer.write(HeightChanged(current_height));
    }
}

pub fn update_height_ui(
    mut reader: MessageReader<HeightChanged>,
    mut text_query: Query<&mut Text, With<HeightText>>,
) {
    let Ok(mut text) = text_query.single_mut() else {
        return;
    };

    for msg in reader.read() {
        **text = format!("Height: {}", msg.0 as i32);
    }
}