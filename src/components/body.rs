use bevy_ecs::prelude::*;
use serde::{Serialize, Deserialize};


#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EquipmentSlot {
	HandRight,
	HandLeft,
	Helmet,
	Necklace,
	Shirt,
	Pants,
}

// See items::Equipped for how equipment slots get assigned.

#[derive(Clone, Component, Debug, PartialEq, Serialize, Deserialize)]
pub struct BodyPart {
	pub slot: EquipmentSlot,
}

#[derive(Clone, Component, Debug, PartialEq, Serialize, Deserialize)]
pub struct Metabolism {
	hunger_clock: i32,
}