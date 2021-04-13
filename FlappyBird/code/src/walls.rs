use gdnative::prelude::*;
use gdnative::api::StaticBody2D;

/// the walls "class"
#[derive(NativeClass)]
#[inherit(StaticBody2D)]
#[register_with(Self::register_walls)]
pub struct Walls {
    position: Vector2,
}

#[methods]
impl Walls {
    fn register_walls(builder: &ClassBuilder<Self>) {
        builder.add_signal(Signal {
            name: "spawn_walls",
            args: &[],
        });
    }

    fn new(_owner: &StaticBody2D) -> Self {
        Walls {
            position: Vector2::new(0.0, 0.0),
        }
    }

    #[export]
    fn _ready(&mut self, _owner: &StaticBody2D) {
        godot_print!("i got wallhacks");
    }

    #[export]
    fn _physics_process(&mut self, _owner: &StaticBody2D, _delta: f32) {
        self.position.x += -2.0;
    }
}


