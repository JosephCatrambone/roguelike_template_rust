use crate::action::Action;
use std::collections::{HashMap, HashSet};
use bevy_ecs::prelude::Resource;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, PartialEq, Resource, Serialize, Deserialize)]
pub struct InputState {
	keymap: HashMap<char, Action>,
	key_lookup: HashMap<Action, char>,
	#[serde(skip)]
	keys_pressed: HashSet<char>,
	#[serde(skip)]
	keys_just_pressed: HashSet<char>,
	#[serde(skip)]
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

	pub fn any_keys_just_pressed(&self) -> bool {
		!self.keys_just_pressed.is_empty()
	}

	pub fn clear_press_states(&mut self) {
		self.keys_just_pressed.clear();
		self.keys_just_released.clear();
	}

	pub fn bind_key(&mut self, key: char, action: Action) {
		self.keymap.remove(&key);
		self.key_lookup.remove(&action);
		self.keymap.insert(key, action.clone());
		self.key_lookup.insert(action, key);
	}

	pub fn handle_key_down(&mut self, key: char) {
		if !self.keys_pressed.contains(&key) {
			self.keys_just_pressed.insert(key);
		}
		self.keys_pressed.insert(key);
	}

	pub fn handle_key_up(&mut self, key: char) {
		if self.keys_pressed.contains(&key) {
			self.keys_just_released.insert(key);
		}
		self.keys_pressed.remove(&key);
	}

	pub fn pop_actions(&mut self) -> Vec<Action> {
		let mut actions = vec![];
		for k in &self.keys_just_pressed {
			if let Some(act) = self.keymap.get(k) {
				actions.push(act.clone());
			}
		}
		self.clear_press_states();
		actions
	}
}