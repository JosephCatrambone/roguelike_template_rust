// Split out into a lib because we want to run unit tests.
mod components;

use legion::*;
use std::rc::Weak;


enum GameMode {
    Paused,
    AwaitPlayerInput,
    BlockingModalCharacter,
    BlockingModelString,
    WorldTick,
}

enum Event {
	Move,
	Drink,
	BeginSleeping,
}

struct GameNode {
	parent: Weak<GameNode>,
	children: Vec<GameNode>,
	fg_codepoint: Option<u32>,
	bg_codepoint: Option<u32>,
	//capabilities: HashMap<Capabilities, >,
	//characteristics: Vec<String>,
}

impl GameNode {
	fn process_event(event: Event) -> Option<Event> {
		// We may pass this to our children.  We may just ignore it and drop it.
		// If we return Some(Event) then it will continue down the chain of actors.
		// There's no ordering guarantee.
		todo!()
	}
}


pub struct GameState {
	ecs_world: World,
	ecs_scheduler: Schedule,
	ecs_resources: Resources,

	map: Vec<GameNode>,
}

impl GameState {
	pub fn new() -> Self {
		let mut schedule = Schedule::builder()
			//.add_system(update_positions_system())
			.build();

		GameState {
			ecs_world: World::default(),
			ecs_scheduler: schedule,
			ecs_resources: Resources::default(),
			map: vec![],
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
