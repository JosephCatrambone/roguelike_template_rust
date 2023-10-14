use crate::color::RGB8;
use std::ops::{Deref, DerefMut};
use bevy_ecs::prelude::*;

// If we're wrapping a single item, this lets us treat them as basically a type alias.
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

#[derive(Copy, Clone, Component, Debug, PartialEq, Eq, Hash)]
pub struct Player;

#[derive(Copy, Clone, Component, Debug, PartialEq, Eq, Hash)]
pub struct PlayerControlled;

#[derive(Clone, Copy, Component, Debug, PartialEq)]
pub struct Position {
	pub x: u32,
	pub y: u32,
}

#[derive(Clone, Copy, Component, Debug, PartialEq)]
pub struct TryMove {
	pub dx: i32, // Will get reset to zero after the move attempt.
	pub dy: i32,
	pub bonk: bool, // True if the last move attempt failed.
}

#[derive(Clone, Copy, Component, Debug, PartialEq)]
pub struct BlocksTile; // Cannot overlap with anything that has this component.

#[derive(Clone, Copy, Component, Debug, PartialEq)]
pub struct Renderable {
	pub codepoint: u32,
	pub fg_color: RGB8,
	pub bg_color: RGB8,
}

#[derive(Clone, Copy, Component, Debug, PartialEq)]
pub struct Tint {
	pub color: RGB8,
	pub mix: bool,
	pub add: bool,
}

#[derive(Clone, Copy, Component, Debug)]
pub struct Hidden;

#[derive(Clone, Component, Debug, PartialEq, PartialOrd)]
pub struct Volume(f32);
derive_derefs!(Volume, f32);

#[derive(Clone, Component, Debug, PartialEq)]
pub struct Viewshed {
	pub visible_tiles : Vec<Position>,
	pub range : u32,
	pub last_computed: Position, // If we change position we need to recompute this.
}

#[derive(Clone, Component, Debug, PartialEq)]
pub struct BodyPart {
	// Should this be an entity?

}