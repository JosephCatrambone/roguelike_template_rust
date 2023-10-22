use bevy_ecs::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Action {
	Move(i32, i32),
	Save,
	Load,
	Interact(Option<Entity>),
	InteractLocation(i32, i32),
	Use(Option<Entity>),
	UseLocation(i32, i32),
	Say(String),
	Examine(Option<Entity>),
	ExamineLocation(i32, i32),
	// Target(Box<Action>, i8, i8), // Target with action?
	Macro(Vec<Action>),
}