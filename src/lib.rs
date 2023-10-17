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
mod systems;

use std::time::{Duration, Instant};
use bevy_ecs::prelude::*;
use bevy_ecs::system::SystemState;
use crate::action::Action;
use crate::color::RGB8;
use crate::components::*;
use crate::input::*;

#[derive(Copy, Clone, Resource, PartialEq, Hash, Eq)]
pub enum RunState {
	AwaitingPlayerAction,
	AwaitingPlayerInventoryInput, // TODO: Perhaps we can do something fancier later.
	Ticking,
}

#[derive(Clone, Resource, PartialEq, Hash)]
pub struct WorldTick {
	tick: u64,
	time_to_next_tick: Duration,
	last_tick_time: Instant,
}

pub struct GameState {
	world: World,
	schedule: Schedule,
	// Map is in resources.
	input_state: InputState,
}

impl GameState {
	pub fn new() -> Self {
		// Set up keymap:
		let mut keymap = InputState::new();
		keymap.bind_key('w', Action::Move(0, -1));
		keymap.bind_key('a', Action::Move(-1, 0));
		keymap.bind_key('s', Action::Move(0, 1));
		keymap.bind_key('d', Action::Move(1, 0));

		// Setup world:
		let mut world = World::default();

		// Insert all the resources:
		world.insert_resource::<map::Map>(map::Map::new_random(600, 500, None));
		world.insert_resource::<RunState>(RunState::AwaitingPlayerAction);
		world.insert_resource::<WorldTick>(WorldTick { tick: 0, time_to_next_tick: Duration::from_secs(2), last_tick_time: Instant::now() });
		world.insert_resource::<gamelog::GameLog>(gamelog::GameLog::default());
		world.insert_resource::<camera::Camera>(camera::Camera::new(300, 200, 80, 60));
		world.insert_resource::<systems::RenderedMap>(systems::RenderedMap::default());

		// Set the run order for our systems:
		let mut schedule = Schedule::default();
		schedule.add_systems((
				systems::update_initiative,
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
			Viewshed::new(40),
			Renderable { codepoint: '@' as u32, fg_color: RGB8::new(0, 255, 128), bg_color: RGB8::new(0, 0, 0) },
		));

		GameState {
			world,
			schedule,
			input_state: keymap
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

	pub fn with_rendered_map_data(&self, render_fn: impl Fn(&systems::RenderedMap) -> ()) {
		let map_data = self.world.get_resource::<systems::RenderedMap>().unwrap();
		render_fn(&map_data);
	}

	pub fn update(&mut self) {
		let current_game_mode = {
			self.world.get_resource::<RunState>().expect("GameMode resource detached!? This can never happen.").clone()
		};
		let next_game_mode = match current_game_mode {
			RunState::AwaitingPlayerAction => {
				self.handle_player_action()
			},
			RunState::AwaitingPlayerInventoryInput => {
				RunState::AwaitingPlayerAction // TODO
			},
			RunState::Ticking => {
				self.schedule.run(&mut self.world); // We have to step the world so that the inputs will be registered.
				RunState::AwaitingPlayerAction
			}
		};
		//self.ecs_world.get_resource::<GameMode>().expect("GameMode resource detached!? This can never happen.").as_ref()
		*(self.world.get_resource_mut::<RunState>().expect("Failed to get game mode!?").as_mut()) = next_game_mode;
	}

	pub fn save(&self) {
	}

	pub fn load(&mut self) {
	}

	// Thin wrappers:
	pub fn handle_key_down(&mut self, key: char) {
		self.input_state.handle_key_down(key);
	}

	pub fn handle_key_up(&mut self, key: char) {
		self.input_state.handle_key_up(key);
	}

	pub fn handle_player_action(&mut self) -> RunState {
		let actions = self.input_state.pop_actions();
		if actions.is_empty() {
			return RunState::AwaitingPlayerAction;
		}
		// TODO: Use 'with' here to speed up the select.
		let mut system_state: SystemState<(Commands, Query<(Entity, Option<&mut TryMove>, &Position, &PlayerControlled)>)> = SystemState::new(&mut self.world);
		let (mut commands, mut query) = system_state.get_mut(&mut self.world);
		for a in actions {
			match a {
				Action::Move(dx, dy) => {
					for (e, maybe_trymove, _pos, _pc) in query.iter_mut() {
						if let Some(mut tm) = maybe_trymove {
							tm.dx = dx as i32;
							tm.dy = dy as i32;
						} else {
							commands.entity(e).insert(TryMove { dx: dx as i32, dy: dy as i32, bonk: false });
						}
					}
				},
				Action::Macro(ops) => {

				},
			}
		}
		system_state.apply(&mut self.world);
		return RunState::Ticking;
	}
}
