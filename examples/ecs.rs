use sepia::app::*;
use specs::{
    Builder, Component, DispatcherBuilder, Join, Read, ReadStorage, System, VecStorage, World,
    WorldExt, WriteStorage,
};

#[derive(Default)]
struct DeltaTime(f32);

#[derive(Component, Debug)]
#[storage(VecStorage)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
struct Velocity {
    x: f32,
    y: f32,
}

struct HelloWorld;

impl<'a> System<'a> for HelloWorld {
    type SystemData = ReadStorage<'a, Position>;
    fn run(&mut self, position: Self::SystemData) {
        for position in position.join() {
            println!("Hello, {:?}", &position);
        }
    }
}

struct UpdatePos;

impl<'a> System<'a> for UpdatePos {
    type SystemData = (
        Read<'a, DeltaTime>,
        ReadStorage<'a, Velocity>,
        WriteStorage<'a, Position>,
    );
    fn run(&mut self, (delta, vel, mut pos): Self::SystemData) {
        let delta = delta.0;
        for (vel, pos) in (&vel, &mut pos).join() {
            pos.x += vel.x * delta;
            pos.y += vel.y * delta;
        }
    }
}

fn main() {
    let mut world = World::new();
    world.register::<Position>();
    world.register::<Velocity>();

    // Add a resource which is data accessible from all systems
    world.insert(DeltaTime(0.05));

    // Write resources
    {
        let mut delta = world.write_resource::<DeltaTime>();
        *delta = DeltaTime(0.04);
    }

    // this does not get a position update because this does not have a velocity
    world
        .create_entity()
        .with(Position { x: 4.0, y: 7.0 })
        .build();
    world
        .create_entity()
        .with(Position { x: 4.0, y: 7.0 })
        .with(Velocity { x: 0.1, y: 0.2 })
        .build();

    let mut dispatcher = DispatcherBuilder::new()
        .with(HelloWorld, "hello_world", &[])
        .with(UpdatePos, "update_pos", &["hello_world"])
        .with(HelloWorld, "hello_updated", &["update_pos"])
        .build();
    dispatcher.dispatch(&mut world);

    // let mut hello_world = HelloWorld;
    // hello_world.run_now(&world);
    // world.maintain();

    // let mut state: EmptyState = EmptyState;
    // let mut state_machine: Vec<&mut dyn State> = Vec::new();
    // state_machine.push(&mut state);
    // App::new(state_machine).run();
}
