

Scratch nodes and stuff:
```
// a component is any type that is 'static, sized, send and sync
#[derive(Clone, Copy, Debug, PartialEq)]
struct Position {
    x: f32,
    y: f32,
}
#[derive(Clone, Copy, Debug, PartialEq)]
struct Velocity {
    dx: f32,
    dy: f32,
}

// push a component tuple into the world to create an entity
let entity: Entity = world.push((Position { x: 0.0, y: 0.0 }, Velocity { dx: 0.0, dy: 0.0 }));

// or extend via an IntoIterator of tuples to add many at once (this is faster)
let entities: &[Entity] = world.extend(vec![
    (Position { x: 0.0, y: 0.0 }, Velocity { dx: 0.0, dy: 0.0 }),
    (Position { x: 1.0, y: 1.0 }, Velocity { dx: 0.0, dy: 0.0 }),
    (Position { x: 2.0, y: 2.0 }, Velocity { dx: 0.0, dy: 0.0 }),
]);


// entries return `None` if the entity does not exist
if let Some(mut entry) = world.entry(entity) {
    // access information about the entity's archetype
    println!("{:?} has {:?}", entity, entry.archetype().layout().component_types());

    // add an extra component
    entry.add_component(12f32);

    // access the entity's components, returns `None` if the entity does not have the component
    assert_eq!(entry.get_component::<f32>().unwrap(), &12f32);
}

// QUERIES:

// you define a query be declaring what components you want to find, and how you will access them
let mut query = <&Position>::query();

// you can then iterate through the components found in the world
for position in query.iter(&world) {
    println!("{:?}", position);
}

// construct a query from a "view tuple"
let mut query = <(&Velocity, &mut Position)>::query();

// this time we have &Velocity and &mut Position
for (velocity, position) in query.iter_mut(&mut world) {
    position.x += velocity.x;
    position.y += velocity.y;
}

// you can use boolean expressions when adding filters
let mut query = <(&Velocity, &mut Position)>::query()
    .filter(!component::<Ignore>() & maybe_changed::<Position>());

for (velocity, position) in query.iter_mut(&mut world) {
    position.x += velocity.dx;
    position.y += velocity.dy;
}
```