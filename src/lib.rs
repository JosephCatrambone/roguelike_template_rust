// Split out into a lib because we want to run unit tests.

mod action;
mod camera;
mod color;
mod components;
mod entities;
mod gamelog;
mod input;
mod map;
mod raycast;
mod rect_tools;
mod saveload;
mod systems;

use bevy_ecs::prelude::*;
use bevy_ecs::system::SystemState;
use bevy_reflect::prelude::*;
use crate::action::Action;
use crate::color::RGB8;
use crate::components::*;
use crate::input::*;
use serde::{Serialize, Deserialize};
use std::time::{Duration, Instant};

#[derive(Copy, Clone, Debug, Resource, PartialEq, Hash, Eq)]
pub enum RunState {
	AwaitingPlayerInput,
	AwaitingPlayerInventoryInput, // TODO: Perhaps we can do something fancier later.
	GenerateMap,
	Menu, // Menu stack pending.
	Saving,
	Loading,
	Ticking,
}

#[derive(Clone, Debug, Resource, PartialEq, Hash)]
pub struct WorldTick {
	tick: u64,
	time_to_next_tick: Duration,
	last_tick_time: Instant,
}

pub struct GameState {
	input_state: InputState,
	world: World,
	schedule: Schedule,
	// Map is in resources.
}

impl GameState {
	pub fn new() -> Self {
		// Set up keymap:
		let mut input_state = InputState::new();
		input_state.bind_key('w', Action::Move(0, -1));
		input_state.bind_key('a', Action::Move(-1, 0));
		input_state.bind_key('s', Action::Move(0, 1));
		input_state.bind_key('d', Action::Move(1, 0));
		input_state.bind_key('c', Action::Save);
		input_state.bind_key('v', Action::Load);

		// Setup world:
		let mut world = World::default();

		// Insert all the resources:
		world.init_resource::<AppTypeRegistry>(); // For saving.
		world.insert_resource::<map::Map>(map::Map::new_random(600, 500, None));
		world.insert_resource::<RunState>(RunState::Ticking);
		world.insert_resource::<WorldTick>(WorldTick { tick: 0, time_to_next_tick: Duration::from_secs(2), last_tick_time: Instant::now() });
		world.insert_resource::<gamelog::GameLog>(gamelog::GameLog::default());
		world.insert_resource::<camera::Camera>(camera::Camera::new(300, 200, 80, 60));
		world.insert_resource::<systems::RenderedMap>(systems::RenderedMap::default());

		// Set the run order for our systems:
		let mut schedule = Schedule::default();
		schedule.add_systems((
				systems::update_initiative,
				systems::ai::npc_thinking,
				systems::step_try_move,
				systems::camera_follow,
				systems::compute_viewshed,
				systems::render_map,
			) //.run_if(step_world),
		);

		// TODO: We are inserting the player.  Hack-ish.
		let _player = world.spawn((
			Position { x: 10, y: 10 },
			Player {},
			PlayerControlled {},
			BlocksTile {},
			Initiative { current: 0 },
			Viewshed::new(40),
			Renderable { codepoint: '@' as u32, fg_color: RGB8::new(0, 255, 128), bg_color: RGB8::new(0, 0, 0) },
		));

		let _rando = world.spawn((
			Position { x: 9, y: 7 },
			ai::NPC::default(),
			Initiative { current: 2 },
			Renderable { codepoint: 'A' as u32, fg_color: RGB8::new(150, 150, 0), bg_color: RGB8::new(0, 0, 0) },
		));

		GameState {
			input_state,
			world,
			schedule,
		}
	}

	// Sets the view frustum of the data block for map rendering and the camera frustum.
	pub fn set_camera_viewport_size(&mut self, width: u32, height: u32) {
		{
			let mut camera = self.world.get_resource_mut::<camera::Camera>().expect("Couldn't get camera ref.");
			camera.width = width;
			camera.height = height;
		}
		{
			let mut map_data = self.world.get_resource_mut::<systems::RenderedMap>().expect("Couldn't get map data ref.");
			map_data.reallocate(width, height);
		}
	}

	fn get_run_state(&self) -> RunState {
		self.world.get_resource::<RunState>().unwrap().clone()
	}

	fn set_run_state(&mut self, new_state: RunState) {
		let mut runstate = self.world.get_resource_mut::<RunState>().unwrap();
		*runstate = new_state;
	}

	pub fn with_rendered_map_data(&self, render_fn: impl Fn(&systems::RenderedMap) -> ()) {
		let map_data = self.world.get_resource::<systems::RenderedMap>().unwrap();
		render_fn(&map_data);
	}

	pub fn update(&mut self) {
		//let start_state = self.world.get_resource::<RunState>().unwrap().clone();
		self.process_player_inputs();
		self.schedule.run(&mut self.world); // We have to step the world so that the inputs will be registered.
	}

	pub fn handle_key_down(&mut self, key: char) {
		// 27, 8, 13
		// Esc, backspace, return.
		self.input_state.handle_key_down(key);
	}

	pub fn handle_key_up(&mut self, key: char) {
		self.input_state.handle_key_up(key);
	}

	pub fn process_player_inputs(&mut self) {
		let old_run_state = self.get_run_state();
		if old_run_state == RunState::Ticking {
			return;
		}

		let mut new_run_state: Option<RunState> = None;
		if old_run_state == RunState::AwaitingPlayerInput {
			// We may get multiple actions but we can only process one if it's the player's turn.
			let player_actions = self.input_state.pop_all_actions();
			if let Some(action) = player_actions.first() {
				match action {
					Action::Move(dx, dy) => {
						let dx = dx.clone();
						let dy = dy.clone();
						let mut system_state: SystemState<(Commands, Query<(Entity, Option<&mut TryMove>, &Position, &PlayerControlled)>)> = SystemState::new(&mut self.world);
						let (mut commands, mut query) = system_state.get_mut(&mut self.world);
						self.input_state.clear_keys();
						for (e, trymove, _pos, _pc) in query.iter_mut() {
							if let Some(mut tm) = trymove {
								tm.dx = dx;
								tm.dy = dy;
							} else {
								commands.entity(e).insert(TryMove { dx, dy });
							}
							new_run_state = Some(RunState::Ticking);
						}
						system_state.apply(&mut self.world);
					},
					Action::Save => {
						println!("{}", saveload::save(&mut self.world));
					}
					_ => {}
				}
			}
		}

		// We may have taken an action.  We may not have.
		if let Some(new_state) = new_run_state {
			self.set_run_state(new_state);
		}
	}

}
