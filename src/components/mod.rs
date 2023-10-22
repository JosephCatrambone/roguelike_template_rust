pub mod ai;
mod body;
mod combat;
mod items;
pub use body::*;
pub use combat::*;
pub use items::*;

use crate::color::RGB8;
use bevy_ecs::prelude::*;
use bevy_reflect::{Reflect, TypeRegistry};
use serde::{Serialize, Deserialize};
use std::ops::{Deref, DerefMut};

pub fn register_components(registry: &mut TypeRegistry) {
	registry.register::<u32>();
	registry.register::<Position>();
	//registry.register_type_data::<Position>();
}

// If we're wrapping a single item, this lets us treat them as basically a type alias.
// Example: pub struct Volume(f32); derive_derefs!(Volume, f32);
macro_rules! derive_derefs {
	($newtype:ident, $oldtype:ident) => {
		impl Deref for $newtype {
			type Target = $oldtype;
			fn deref(&self) -> &Self::Target {
				&self.0
			}
		}

		impl DerefMut for $newtype {
			fn deref_mut(&mut self) -> &mut Self::Target {
				&mut self.0
			}
		}
    };
}

#[derive(Clone, Component, Debug, PartialEq, Eq, Hash, Reflect, Serialize, Deserialize)]
pub struct Name(String);
derive_derefs!(Name, String);

impl Into<String> for Name {
	fn into(self) -> String {
		self.0
	}
}


#[derive(Copy, Clone, Component, Debug, PartialEq, Eq, Hash, Reflect, Serialize, Deserialize)]
pub struct Initiative {
	pub current: i32,
}

#[derive(Copy, Clone, Component, Debug, PartialEq, Eq, Hash, Reflect, Serialize, Deserialize)]
pub struct TurnActive;

#[derive(Copy, Clone, Component, Debug, PartialEq, Eq, Hash, Reflect, Serialize, Deserialize)]
pub struct Player;

#[derive(Copy, Clone, Component, Debug, PartialEq, Eq, Hash, Reflect, Serialize, Deserialize)]
pub struct PlayerControlled;

#[derive(Clone, Copy, Component, Debug, PartialEq, Reflect, Serialize, Deserialize)]
pub struct Position {
	pub x: u32,
	pub y: u32,
}

#[derive(Clone, Copy, Component, Debug, PartialEq, Reflect, Serialize, Deserialize)]
pub struct TryMove {
	pub dx: i32,
	pub dy: i32,
}

#[derive(Clone, Copy, Component, Debug, PartialEq, Reflect, Serialize, Deserialize)]
pub struct MoveFailed;

#[derive(Clone, Copy, Component, Debug, PartialEq, Reflect, Serialize, Deserialize)]
pub struct MoveSucceeded;

#[derive(Clone, Copy, Component, Debug, PartialEq, Reflect, Serialize, Deserialize)]
pub struct BlocksTile; // Cannot overlap with anything that has this component.

#[derive(Clone, Copy, Component, Debug, PartialEq, Reflect, Serialize, Deserialize)]
pub struct Renderable {
	pub codepoint: u32,
	pub fg_color: RGB8,
	pub bg_color: RGB8,
}

#[derive(Clone, Copy, Component, Debug, PartialEq, Reflect, Serialize, Deserialize)]
pub struct Tint {
	pub color: RGB8,
	pub mix: bool,
	pub add: bool,
}

#[derive(Clone, Copy, Component, Debug, Reflect, Serialize, Deserialize)]
pub struct Hidden;

#[derive(Clone, Component, Debug, PartialEq, Reflect, Serialize, Deserialize)]
pub struct Viewshed {
	pub visible_tiles : Vec<Position>,
	pub range : u32,
	pub last_computed: Position, // If we change position we need to recompute this.
}

impl Viewshed {
	pub fn new(sight_range: u32) -> Self {
		Self {
			visible_tiles: vec![],
			last_computed: Position { x: 0, y: 0 },
			range: sight_range,
		}
	}
}
