use bevy_ecs::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Component, Debug, PartialEq, Serialize, Deserialize)]
pub struct Item;

#[derive(Clone, Component, Debug, PartialEq, Serialize, Deserialize)]
pub struct InInventory {
	pub owner: Entity
}

#[derive(Clone, Component, Debug, PartialEq, Serialize, Deserialize)]
pub struct WantsToUseItem {
	pub item: Entity
}

#[derive(Clone, Component, Debug, PartialEq, Serialize, Deserialize)]
pub struct WantsToDropItem {
	pub item: Entity
}