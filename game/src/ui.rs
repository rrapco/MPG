use bevy::prelude::*;
use crate::player::Player;
use crate::gamestate::InGameEntity;

#[derive(Message, Clone)]
pub struct HeightChanged(pub f32);

#[derive(Component)]
pub struct HeightText;

#[derive(Component)]
pub struct TimerText;

#[derive(Resource)]
pub struct LevelTimer {
    pub seconds: f32,
}

pub fn setup_ui(mut commands: Commands) {
    commands.insert_resource(LevelTimer { seconds: 0.0 });

    commands.spawn((
        Text::new("Time: 0.00"),
        TextFont {
            font_size: 22.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(12.0),
            left: Val::Px(12.0),
            ..default()
        },
        TimerText,
        InGameEntity,
    ));

    commands.spawn((
        Text::new("Height: 0"),
        TextFont {
            font_size: 18.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(40.0),
            left: Val::Px(12.0),
            ..default()
        },
        HeightText,
        InGameEntity,
    ));
}

pub fn update_timer_ui(
    time: Res<Time>,
    mut level_timer: ResMut<LevelTimer>,
    mut text_query: Query<&mut Text, With<TimerText>>,
) {
    level_timer.seconds += time.delta_secs();

    let Ok(mut text) = text_query.single_mut() else {
        return;
    };

    **text = format!("Time: {:.2}", level_timer.seconds);
}

pub fn detect_height_change(
    player_query: Query<&Transform, With<Player>>,
    mut last_height: Local<f32>,
    mut writer: MessageWriter<HeightChanged>,
) {
    let Ok(transform) = player_query.single() else {
        return;
    };

    let current_height = (((transform.translation.y) / 10.0) - 10.0).round();

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