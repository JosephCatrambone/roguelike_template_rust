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

#[derive(Copy, Clone, Debug, Resource, PartialEq, Hash, Eq)]
pub enum RunState {
	AwaitingPlayerAction,
	AwaitingPlayerInventoryInput, // TODO: Perhaps we can do something fancier later.
	Ticking,
}

#[derive(Clone, Debug, Resource, PartialEq, Hash)]
pub struct WorldTick {
	tick: u64,
	time_to_next_tick: Duration,
	last_tick_time: Instant,
}

pub struct GameState {
	world: World,
	schedule: Schedule,
	// Map is in resources.
	// So is input_state.
}

impl GameState {
	pub fn new() -> Self {
		// Set up keymap:
		let mut input_state = InputState::new();
		input_state.bind_key('w', Action::MoveUp);
		input_state.bind_key('a', Action::MoveLeft);
		input_state.bind_key('s', Action::MoveDown);
		input_state.bind_key('d', Action::MoveRight);

		// Setup world:
		let mut world = World::default();

		// Insert all the resources:
		world.insert_resource::<InputState>(input_state);
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
				systems::player_movement_input,
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

	pub fn with_rendered_map_data(&self, render_fn: impl Fn(&systems::RenderedMap) -> ()) {
		let map_data = self.world.get_resource::<systems::RenderedMap>().unwrap();
		render_fn(&map_data);
	}

	pub fn update(&mut self) {
		let start_state = self.world.get_resource::<RunState>().unwrap().clone();
		self.schedule.run(&mut self.world); // We have to step the world so that the inputs will be registered.
		let end_state = self.world.get_resource::<RunState>().unwrap().clone();

		if start_state == RunState::AwaitingPlayerAction && end_state != RunState::AwaitingPlayerAction {
			// We processed the inputs!  Clear them.
			let mut inputs = self.world.get_resource_mut::<InputState>().unwrap();
			inputs.clear_keys();
		}
	}

	pub fn save(&self) {
	}

	pub fn load(&mut self) {
	}

	// Thin wrappers:
	pub fn handle_key_down(&mut self, key: char) {
		self.world.get_resource_mut::<InputState>().expect("Lost input state.").handle_key_down(key);
	}

	pub fn handle_key_up(&mut self, key: char) {
		self.world.get_resource_mut::<InputState>().expect("Lost input state.").handle_key_up(key);
	}

	/*
	let mut system_state: SystemState<(Commands, Query<(Entity, Option<&mut TryMove>, &Position, &PlayerControlled)>)> = SystemState::new(&mut self.world);
	let (mut commands, mut query) = system_state.get_mut(&mut self.world);
	for (e, maybe_trymove, _pos, _pc) in query.iter_mut() { ...
	commands.entity(e).insert(TryMove { dx: dx as i32, dy: dy as i32, bonk: false });

	system_state.apply(&mut self.world);
	*/
}
