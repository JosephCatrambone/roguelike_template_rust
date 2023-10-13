// Split out into a lib because we want to run unit tests.

mod color;
mod components;
mod map;
mod raycast;
mod systems;

use crate::components::Position;
use legion::*;
use legion::systems::*;


enum GameMode {
    Paused,
    AwaitPlayerInput,
    BlockingModalCharacter,
    BlockingModelString,
    WorldTick,
}

pub struct GameState {
	ecs_world: World,
	ecs_scheduler: Schedule,
	ecs_resources: Resources,

	map: map::Map,
}

impl GameState {
	pub fn new() -> Self {
		let mut schedule = Schedule::builder()
			//.add_system(update_positions_system())
			.add_system(systems::viewshed_system::compute_viewshed_system())
			.build();

		GameState {
			ecs_world: World::default(),
			ecs_scheduler: schedule,
			ecs_resources: Resources::default(),
			map: map::Map::new(),
		}
	}

	pub fn update(&mut self) {
		self.ecs_scheduler.execute(&mut self.ecs_world, &mut self.ecs_resources);
	}
}


pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
