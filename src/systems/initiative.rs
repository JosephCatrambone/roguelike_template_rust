use std::time::{Duration, Instant};
use bevy_ecs::prelude::*;
use rand::{RngCore, thread_rng};

use crate::components::{Initiative, TurnActive, PlayerControlled};
use crate::{RunState, WorldTick};

const SOFT_REALTIME: bool = false;
const MAX_RANDOM_INITIATIVE: u32 = 5;

pub fn update_initiative(mut commands: Commands, mut query: Query<(Entity, &mut Initiative, Option<&PlayerControlled>)>, mut run_state: ResMut<RunState>, mut world_tick: ResMut<WorldTick>) {
	if SOFT_REALTIME {
		// If we want to do some soft-realtime stuff we can turn off the checks above and turn on these:
		// Tick the world.
		let now = Instant::now();
		let delta_time = now - world_tick.last_tick_time;
		world_tick.time_to_next_tick = world_tick.time_to_next_tick.saturating_sub(delta_time);
		world_tick.last_tick_time = now;
		if world_tick.time_to_next_tick.as_secs_f32() > 0.0 {
			// Done early!
			return;
		}
		world_tick.time_to_next_tick = Duration::from_secs(1);
	} else {
		// This will mean we don't get any real-time events like things moving around the player.  Perhaps we could say EITHER we're waiting for a player input or the world is ticking?
		if run_state.as_ref() != &RunState::Ticking {
			return;
		}
	}

	// Otherwise we have to tick and update the things like initiative.
	let mut rng = thread_rng();
	for (e, mut init, maybe_pc) in query.iter_mut() {
		init.current -= 1;
		if init.current <= 0 {
			init.current += (rng.next_u32() % MAX_RANDOM_INITIATIVE) as i32;
			commands.entity(e).insert(TurnActive); // Make active and bump our initiative!
			if maybe_pc.is_some() {
				*run_state = RunState::AwaitingPlayerInput;
			}
		}
		// TODO: Pop any superfluous TurnActives?
	}
}