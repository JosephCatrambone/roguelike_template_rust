use crate::components::*;
use crate::map::Map;
use bevy_ecs::prelude::*;
use crate::action::Action;
use crate::input::InputState;
use crate::RunState;

pub fn step_try_move(mut commands: Commands, mut query: Query<(Entity, &mut Position, &mut TryMove), With<TurnActive>>, map: Res<Map>) {
	for (e, mut pos, mut vel) in query.iter_mut() {
		let old_x = pos.x;
		let old_y = pos.y;
		if vel.dx == 0 && vel.dy == 0 {
			// If this entity is not moving then don't burn the action on it.
			commands.entity(e)
				.remove::<TryMove>()
				.remove::<MoveFailed>()
				.remove::<MoveSucceeded>();
			continue;
		}
		pos.x = pos.x.saturating_add_signed(vel.dx);
		pos.y = pos.y.saturating_add_signed(vel.dy);
		if !map.tile_open(pos.x, pos.y) {
			pos.x = old_x;
			pos.y = old_y;
			commands.entity(e)
				.insert(MoveFailed)
				.remove::<MoveFailed>();
		}
		vel.dx = 0;
		vel.dy = 0;
		commands.entity(e)
			.remove::<TurnActive>()
			.insert(MoveSucceeded);
	}
}

pub fn player_movement_input(mut commands: Commands, mut query: Query<(Entity, &mut Position, Option<&mut TryMove>), With<PlayerControlled>>, mut run: ResMut<RunState>, mut input_state: ResMut<InputState>) {
	if run.as_ref() != &RunState::AwaitingPlayerAction {
		// Early out.  Not our turn for inputs yet.
		return;
	}

	// If it _IS_ our turn now...
	let dy = if input_state.is_action_just_pressed(Action::MoveUp) { -1 } else if input_state.is_action_just_pressed(Action::MoveDown) { 1 } else { 0 };
	let dx = if input_state.is_action_just_pressed(Action::MoveLeft) { -1 } else if input_state.is_action_just_pressed(Action::MoveRight) { 1 } else { 0 };
	if dx != 0 || dy != 0 {
		input_state.clear_keys();
		*run = RunState::Ticking;
		for (e, pos, trymove) in query.iter_mut() {
			if let Some(mut tm) = trymove {
				tm.dx = dx;
				tm.dy = dy;
			} else {
				commands.entity(e).insert(TryMove { dx, dy });
			}
		}
	}
}
