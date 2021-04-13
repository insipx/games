use gdnative::prelude::*;

mod player;
mod obstacle;
mod obstacle_spawner;
mod hud;
mod world;

fn init(handle: InitHandle) {
    handle.add_class::<player::Player>();
    handle.add_class::<obstacle::Obstacle>();
    handle.add_class::<obstacle_spawner::ObstacleSpawner>();
    handle.add_class::<hud::Hud>();
    handle.add_class::<world::World>();
}

godot_init!(init);

