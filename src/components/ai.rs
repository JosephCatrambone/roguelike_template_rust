use bevy_ecs::prelude::*;
use serde::{Serialize, Deserialize};
use crate::action::Action;


#[derive(Clone, Component, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NPC {
	pub selected_action: Option<Action>,
}
