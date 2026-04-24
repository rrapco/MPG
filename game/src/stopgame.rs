use std::fs::{OpenOptions, read_to_string};
use std::io::Write;

use bevy::prelude::*;
use avian2d::prelude::*;
use crate::enemy::Enemy;
use crate::player::Player;
use crate::map::goal::VictoryTimer;
use crate::death::DeathTimer;

pub fn freeze_entities(
    mut query: Query<&mut LinearVelocity, Or<(With<Player>, With<Enemy>)>>,
    dead: Option<Res<DeathTimer>>,
    victory: Option<Res<VictoryTimer>>,
) {
    if dead.is_none() && victory.is_none() {
        return;
    }

    for mut velocity in &mut query {
        velocity.x = 0.0;
        velocity.y = 0.0;
    }
}
pub fn save_level_time(level: usize, time: f32) {
    let path = "times.txt";

    let content = read_to_string(path).unwrap_or_default();
    let mut lines: Vec<String> = content.lines().map(|line| line.to_string()).collect();

    let level_prefix = format!("level{}:", level);
    let new_line = format!("level{}: {:.2}", level, time);

    let mut found = false;

    for line in &mut lines {
        if line.starts_with(&level_prefix) {
            found = true;

            let old_time_text = line
                .split(':')
                .nth(1)
                .unwrap_or("")
                .trim();

            let old_time = old_time_text.parse::<f32>().unwrap_or(f32::MAX);

            if time < old_time {
                *line = new_line.clone();
            }

            break;
        }
    }

    if !found {
        lines.push(new_line);
    }

    let final_text = lines.join("\n");

    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(path)
        .unwrap();

    file.write_all(final_text.as_bytes()).unwrap();
}