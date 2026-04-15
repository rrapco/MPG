use bevy::prelude::*;
use crate::constants::{PLAYER_HEIGHT, PLAYER_WIDTH, TILE_SIZE_X, TILE_SIZE_Y};
use crate::gamestate::{GameState, InGameEntity};
use crate::player::Player;

#[derive(Component)]
pub struct Goal;

#[derive(Component)]
pub struct VictoryText;

#[derive(Resource)]
pub struct VictoryTimer(pub Timer);

pub fn spawn_goal(commands: &mut Commands, x: f32, y: f32) {
    commands.spawn((
        Sprite::from_color(Color::srgb(1.0, 0.9, 0.0), Vec2::new(TILE_SIZE_X, TILE_SIZE_Y)),
        Transform::from_xyz(x, y, 2.0),
        Goal,
        InGameEntity,
    ));
}

fn spawn_victory_ui(commands: &mut Commands) {
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(20.0),
            ..default()
        },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.55)),
        VictoryText,
        InGameEntity,
    ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("VICTORY!"),
                TextFont {
                    font_size: 72.0,
                    ..default()
                },
                TextColor(Color::srgb(1.0, 0.9, 0.2)),
            ));

            parent.spawn((
                Text::new("Press space to return to menu"),
                TextFont {
                    font_size: 28.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
        });
}

pub fn check_goal_collision(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    goal_query: Query<(Entity, &Transform), With<Goal>>,
    victory_timer: Option<Res<VictoryTimer>>,
) {
    if victory_timer.is_some() {
        return;
    }

    let Ok(player_transform) = player_query.single() else {
        return;
    };

    let player_pos = player_transform.translation.truncate();

    for (goal_entity, goal_transform) in &goal_query {
        let goal_pos = goal_transform.translation.truncate();

        let overlap_x = (player_pos.x - goal_pos.x).abs() < (PLAYER_WIDTH / 2.0 + TILE_SIZE_X / 2.0);
        let overlap_y = (player_pos.y - goal_pos.y).abs() < (PLAYER_HEIGHT / 2.0 + TILE_SIZE_Y / 2.0);

        if overlap_x && overlap_y {
            commands.entity(goal_entity).despawn();
            spawn_victory_ui(&mut commands);
            commands.insert_resource(VictoryTimer(Timer::from_seconds(60.0, TimerMode::Once)));
            break;
        }
    }
}

pub fn victory_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
    victory_timer: Option<Res<VictoryTimer>>,
) {
    if victory_timer.is_none() {
        return;
    }

    if keyboard_input.just_pressed(KeyCode::Space) {
        next_state.set(GameState::Menu);
    }
}

pub fn victory_countdown(
    time: Res<Time>,
    victory_timer: Option<ResMut<VictoryTimer>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let Some(mut victory_timer) = victory_timer else {
        return;
    };

    victory_timer.0.tick(time.delta());

    if victory_timer.0.just_finished() {
        next_state.set(GameState::Menu);
    }
}