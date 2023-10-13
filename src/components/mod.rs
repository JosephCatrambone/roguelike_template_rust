use crate::color::RGB8;
use std::ops::{Deref, DerefMut};
use glam::{IVec2, UVec2};

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

pub struct Player;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Position(UVec2);
derive_derefs!(Position, UVec2);

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Velocity(IVec2);
derive_derefs!(Velocity, IVec2);

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TryMove {
	pub dx: i32,
	pub dy: i32,
	pub bonk: bool,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Renderable {
	pub codepoint: char,
	pub fg_color: RGB8,
	pub bg_color: RGB8,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Tint {
	pub color: RGB8,
	pub mix: bool,
	pub add: bool,
}

#[derive(Debug)]
pub struct Volume {
}

#[derive(Clone, Debug, PartialEq)]
pub struct Viewshed {
	pub visible_tiles : Vec<UVec2>,
	pub range : i32,
	pub dirty: bool,
}