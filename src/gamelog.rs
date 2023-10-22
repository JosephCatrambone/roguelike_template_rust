use bevy_ecs::prelude::*;
use crate::components::Position;

#[derive(Default, Resource)]
pub struct GameLog {
	logs: Vec<LogEntry>,
	last_read: usize, // What was the last accessed item?
}

impl GameLog {
	pub fn add_entry(&mut self, text: impl Into<String>) {
		self.logs.push(LogEntry {
			text: text.into(),
			importance: 0.0,
			position: None,
			source: None,
		});
	}

	pub fn tail(&mut self) -> &[LogEntry] {
		let ref_slice = &self.logs[self.last_read..];
		self.last_read = self.logs.len();
		ref_slice
	}
}

#[derive(Default)]
pub struct LogEntry {
	pub text: String,
	pub importance: f32, // 0 is default importance.  Lower is less relevant to the player.  Higher is more relevant.
	pub position: Option<Position>, // If this happened in the world, where?
	pub source: Option<Entity>, // If this was caused by an entity, who?
}