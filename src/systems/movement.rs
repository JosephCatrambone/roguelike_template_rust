use crate::components::*;
use crate::map::Map;
use crate::input::InputState;
use bevy_ecs::prelude::*;
use crate::action::Action;


pub fn step_try_move(mut query: Query<(&mut Position, &mut TryMove)>, map: Res<Map>) {
	for (mut pos, mut vel) in query.iter_mut() {
		let old_x = pos.x;
		let old_y = pos.y;
		pos.x = pos.x.saturating_add_signed(vel.dx);
		pos.y = pos.y.saturating_add_signed(vel.dy);
		vel.bonk = false;
		if !map.tile_open(pos.x, pos.y) {
			pos.x = old_x;
			pos.y = old_y;
			vel.bonk = true;
		}
		vel.dx = 0;
		vel.dy = 0;
	}
}

pub fn player_movement(mut commands: Commands, query: Query<Entity, (With<Position>, With<PlayerControlled>)>, input_state: Res<InputState>) {
	let mut dx = 0;
	let mut dy = 0;
	if input_state.is_action_just_pressed(Action::MoveUp) {
		dy -= 1;
	}
	if input_state.is_action_just_pressed(Action::MoveRight) {
		dx += 1;
	}
	if input_state.is_action_just_pressed(Action::MoveLeft) {
		dx -= 1;
	}
	if input_state.is_action_just_pressed(Action::MoveDown) {
		dy += 1;
	}

	if dx != 0 || dy != 0 {
		for e in query.iter() {
			// This may override the other trymove.
			commands.entity(e).insert(TryMove { dx, dy, bonk: false });
		}
	}
}