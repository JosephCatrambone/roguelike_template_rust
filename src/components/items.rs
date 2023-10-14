use bevy_ecs::prelude::*;
use serde::{Deserialize, Serialize};
use crate::components::EquipmentSlot;

#[derive(Clone, Component, Debug, PartialEq, Serialize, Deserialize)]
pub struct Item;

#[derive(Clone, Component, Debug, PartialEq, Serialize, Deserialize)]
pub struct InInventory {
	pub owner: Entity
}

#[derive(Clone, Component, Debug, PartialEq, Serialize, Deserialize)]
pub struct WantsToUseItem { // Eat / drink / inhale / etc.  Not equip.
	pub item: Entity
}

#[derive(Clone, Component, Debug, PartialEq, Serialize, Deserialize)]
pub struct WantsToDropItem {
	pub item: Entity
}

// This indicates that an item is equipped.  This is perhaps in the middle of items / body.
#[derive(Clone, Component, Debug, PartialEq, Serialize, Deserialize)]
pub struct Equipped {
	pub owner: Entity,
	pub slot: EquipmentSlot,
}
