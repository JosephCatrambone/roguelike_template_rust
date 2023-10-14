use crate::action::Action;
use std::collections::{HashMap, HashSet};
use bevy_ecs::prelude::Resource;

#[derive(Debug, Clone, Default, PartialEq, Resource)]
pub struct InputState {
	keymap: HashMap<char, Action>,
	key_lookup: HashMap<Action, char>,
	keys_pressed: HashSet<char>,
	keys_just_pressed: HashSet<char>,
	keys_just_released: HashSet<char>,
}

impl InputState {
	pub fn new() -> Self {
		InputState {
			keymap: HashMap::new(),
			key_lookup: HashMap::new(),
			keys_pressed: HashSet::new(),
			keys_just_pressed: HashSet::new(),
			keys_just_released: HashSet::new(),
		}
	}

	pub fn is_key_pressed(&self, key: char) -> bool {
		self.keys_pressed.contains(&key)
	}

	pub fn is_action_pressed(&self, action: Action) -> bool {
		if let Some(key) = self.key_lookup.get(&action) {
			self.is_key_pressed(*key)
		} else {
			false
		}
	}

	pub fn is_key_just_pressed(&self, key: char) -> bool {
		self.keys_just_pressed.contains(&key)
	}

	pub fn is_action_just_pressed(&self, action: Action) -> bool {
		if let Some(key) = self.key_lookup.get(&action) {
			self.is_key_just_pressed(*key)
		} else {
			eprintln!("Unrecognized keyaction: {:?}", &action);
			false
		}
	}

	pub fn is_key_just_released(&self, key: char) -> bool {
		self.keys_just_released.contains(&key)
	}

	pub fn is_action_just_released(&self, action: Action) -> bool {
		if let Some(key) = self.key_lookup.get(&action) {
			self.is_key_just_released(*key)
		} else {
			eprintln!("Unrecognized keyaction: {:?}", &action);
			false
		}
	}

	pub fn reset_press_states(&mut self) {
		self.keys_just_pressed.clear();
		self.keys_just_released.clear();
	}

	pub fn bind_key(&mut self, key: char, action: Action) {
		self.keymap.remove(&key);
		self.key_lookup.remove(&action);
		self.keymap.insert(key, action.clone());
		self.key_lookup.insert(action, key);
	}

	pub fn update_from_keys(&mut self, new_keys_down: &HashSet<char>) {
		// Clear the old keys just pressed and released.
		self.keys_just_pressed.clear();
		self.keys_just_released.clear();
		// Update pressed keys:
		for k in new_keys_down.iter() {
			if !self.keys_pressed.contains(k) {
				self.keys_just_pressed.insert(*k);
				self.keys_pressed.insert(*k);
			}
		}
		// Update released keys:
		for k in self.keys_pressed.iter() {
			if !new_keys_down.contains(k) {
				self.keys_just_released.insert(*k);
			}
		}
		// And now clear the released.
		for k in self.keys_just_released.iter() {
			self.keys_pressed.remove(k);
		}
	}
}