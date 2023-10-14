use bevy_ecs::prelude::*;
use crate::components::BodyPart;

pub fn metabolism(mut commands: Commands, query: Query<Entity, (With<BodyPart>)>) {
}