use std::time::Instant;
use bevy_ecs::prelude::*;

use crate::components::{Initiative, TurnActive, PlayerControlled};
use crate::{RunState, WorldTick};

pub fn update_initiative(mut commands: Commands, mut query: Query<(Entity, &mut Initiative, Option<&PlayerControlled>)>, mut run_state: ResMut<RunState>, mut world_tick: ResMut<WorldTick>) {
	// This will mean we don't get any real-time events like things moving around the player.  Perhaps we could say EITHER we're waiting for a player input or the world is ticking?
	if run_state.as_ref() != &RunState::Ticking {
		return;
	}

	// Tick the world.
	let now = Instant::now();
	let delta_time = now - world_tick.last_tick_time;
	world_tick.time_to_next_tick = world_tick.time_to_next_tick.saturating_sub(delta_time);
	world_tick.last_tick_time = now;
	if world_tick.time_to_next_tick.as_secs_f32() > 0.0 {
		// Done early!
		return;
	}

	// Otherwise we have to tick and update the things like initiative.
	for (e, mut init, maybe_pc) in query.iter_mut() {
		init.current -= 1;
		if init.current <= 0 {
			commands.entity(e).insert(TurnActive);
		}
		if maybe_pc.is_some() {
			*run_state = RunState::AwaitingPlayerAction;
		}
	}
}