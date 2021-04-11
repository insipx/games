use gdnative::prelude::*;

/// The player "class"
#[derive(NativeClass)]
#[inherit(Node2D)]
pub struct Player {
    #[property(default = 400.0)]
    velocity: f32,
}

#[methods]
impl Player {
    fn new(_owner: &Node2D) -> Self {
        Player {
            velocity: 400.0,
        }
    }

    #[export]
    fn _ready(&mut self, _owner: &Node2D) {
        godot_print!("What's up motherfuckers!!");
    }
}
