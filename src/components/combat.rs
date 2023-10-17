use bevy_ecs::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Clone, Component, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Health {
	
}