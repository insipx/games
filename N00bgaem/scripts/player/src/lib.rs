use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Node)]
struct Player;

#[methods]
impl Player {
    fn new(_owner: &Node) -> Self {
        Player
    }
    
    #[export]
    fn _ready(&mut self, mut _owner: &Node) {
        godot_print!("What's up, Motherfuckers!!!");
        // owner.set_physics_process(true);
    }
}

fn init(handle: InitHandle) {
    handle.add_class::<Player>();
}

// only need this once in entire lib
godot_init!(init);
