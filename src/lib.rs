// Split out into a lib because we want to run unit tests.

mod action;
mod camera;
mod color;
mod components;
mod gamelog;
mod input;
mod map;
mod raycast;
mod rect_tools;
mod systems;

use std::collections::HashSet;
use crate::input::*;
use crate::components::*;
use bevy_ecs::prelude::*;
use crate::action::Action;
use crate::camera::Camera;
use crate::color::RGB8;


#[derive(Resource)]
enum GameMode {
    Paused,
	AwaitingPlayerPlayInput,
	AwaitingPlayerInventoryInput, // TODO: Perhaps we can do something fancier later.
    WorldTick,
}

pub struct GameState {
	ecs_world: World,
	ecs_scheduler: Schedule,
	// Map is in resources.
	// Input state is also in resources.
}

impl GameState {
	pub fn new() -> Self {
		// Set up keymap:
		let mut keymap = InputState::new();
		keymap.bind_key('w', Action::MoveUp);
		keymap.bind_key('a', Action::MoveLeft);
		keymap.bind_key('s', Action::MoveDown);
		keymap.bind_key('d', Action::MoveRight);

		// Setup world:
		let mut world = World::default();

		// Insert all the resources:
		world.insert_resource::<map::Map>(map::Map::new_random(600, 500, None));
		world.insert_resource::<GameMode>(GameMode::Paused);
		world.insert_resource::<gamelog::GameLog>(gamelog::GameLog::default());
		world.insert_resource::<InputState>(keymap);
		world.insert_resource::<camera::Camera>(camera::Camera::new(300, 200, 80, 60));
		world.insert_resource::<systems::map_rendering::RenderedMap>(systems::map_rendering::RenderedMap::default());

		// Set the run order for our systems:
		let mut schedule = Schedule::default();
		schedule.add_systems(systems::movement::step_try_move);
		schedule.add_systems(systems::movement::player_movement);
		schedule.add_systems(systems::viewshed_system::compute_viewshed);
		schedule.add_systems(systems::map_rendering::render_map);
		schedule.add_systems(systems::camera_follow::camera_follow);

		// TODO: We are inserting the player.  Hack-ish.
		let _player = world.spawn((
			Position { x: 10, y: 10 },
			Player {},
			PlayerControlled {},
			BlocksTile {},
			Viewshed { visible_tiles: vec![], range: 30, last_computed: Position { x: 0, y: 0 } },
			Renderable { codepoint: '@' as u32, fg_color: RGB8::new(0, 255, 128), bg_color: RGB8::new(0, 0, 0) },
		));

		GameState {
			ecs_world: world,
			ecs_scheduler: schedule,
		}
	}

	// Sets the view frustum of the data block for map rendering and the camera frustum.
	pub fn set_camera_viewport_size(&mut self, width: u32, height: u32) {
		{
			let mut camera = self.ecs_world.get_resource_mut::<camera::Camera>().expect("Couldn't get camera ref.");
			camera.width = width;
			camera.height = height;
		}
		{
			let mut map_data = self.ecs_world.get_resource_mut::<systems::map_rendering::RenderedMap>().expect("Couldn't get map data ref.");
			map_data.reallocate(width, height);
		}
	}

	pub fn with_rendered_map_data(&self, render_fn: impl Fn(&systems::map_rendering::RenderedMap) -> ()) {
		let map_data = self.ecs_world.get_resource::<systems::map_rendering::RenderedMap>().unwrap();
		render_fn(&map_data);
	}

	pub fn update(&mut self) {
		// Update systems:
		self.ecs_scheduler.run(&mut self.ecs_world);
	}

	pub fn input(&mut self, keys_down: &HashSet<char>) {
		let mut inputs = self.ecs_world.get_resource_mut::<InputState>().expect("Input state was lost!");
		inputs.update_from_keys(keys_down);
	}

	pub fn save(&self) {
	}

	pub fn load(&mut self) {
	}
}
