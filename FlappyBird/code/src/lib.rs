use gdnative::prelude::*;

mod player;
mod walls;

fn init(handle: InitHandle) {
    handle.add_class::<player::Player>();
    handle.add_class::<walls::Walls>();
}

godot_init!(init);



