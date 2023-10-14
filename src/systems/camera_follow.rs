use bevy_ecs::prelude::*;
use crate::camera::Camera;
use crate::components::{Position, PlayerControlled};

pub fn camera_follow(query: Query<(&Position, &PlayerControlled)>, mut camera: ResMut<Camera>) {
	// Move the camera to the last player controlled object.
	for (pos, _pc) in query.iter() {
		camera.teleport_to(pos.x, pos.y);
	}
}