use std::collections::BTreeMap;
use bevy_ecs::{
	entity::Entity,
	reflect::{AppTypeRegistry, ReflectComponent, ReflectMapEntities},
	world::World,
};
use bevy_ecs::prelude::*;
use bevy_reflect::serde::{ReflectSerializer, UntypedReflectDeserializer};


pub fn save(world: &World) -> String {
	let type_registry = world.resource::<AppTypeRegistry>().read();
	//let mut registry = TypeRegistry::default();
	//register_components(&mut registry);

	let mut serialized_entities = BTreeMap::new();
	for entity_ref in world.iter_entities() {
		let mut entity_serializable = BTreeMap::new();

		for component_id in entity_ref.archetype().components() {
			let maybe_component_info = world.components().get_info(component_id);
			if maybe_component_info.is_none() { continue; }
			let component_info = maybe_component_info.unwrap();

			let maybe_type_id = component_info.type_id();
			if maybe_type_id.is_none() { continue; }
			let type_id = maybe_type_id.unwrap();
			let registration = type_registry.get(type_id);
			if registration.is_none() { continue; }
			let maybe_reflect_component = registration.unwrap().data::<ReflectComponent>();
			if maybe_reflect_component.is_none() { continue; }
			let reflected_ref = maybe_reflect_component.unwrap().reflect(entity_ref).unwrap();

			let serializer = ReflectSerializer::new(reflected_ref, &type_registry);
			//let serialized = ron::ser::to_string_pretty(&serializer, ron::ser::PrettyConfig::default()).unwrap();
			entity_serializable.insert(component_id.index(), serde_json::ser::to_string(&serializer).unwrap());
		}
		serialized_entities.insert(entity_ref.id(), entity_serializable);
	}

	//let reflect_serializer = ReflectSerializer::new(&serialized_entities, &type_registry);
	//let serialized_value: String = serde_json::to_string(&reflect_serializer).unwrap();
	let serialized_value: String = serde_json::to_string(&serialized_entities).unwrap();
	serialized_value
}

pub fn load(world: &mut World, json_string: String) {
	//self.world = serde_json::from_str(&*json_string)?;

	/*
	#[derive(Reflect)]
	struct MyStruct {
	  foo: i32
	}
	let original: Box<dyn Reflect> = Box::new(MyStruct {
	  foo: 123
	});

	let cloned: Box<dyn Reflect> = original.clone_value();
	let value = <MyStruct as FromReflect>::from_reflect(&*cloned).unwrap(); // OK!
	*/

	/*
	// Deserialize
	let reflect_deserializer = UntypedReflectDeserializer::new(&registry);
	let deserialized_value: Box<dyn Reflect> = reflect_deserializer.deserialize(
	  &mut ron::Deserializer::from_str(&serialized_value).unwrap()
	).unwrap();
	*/
}