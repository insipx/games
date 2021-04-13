use gdnative::prelude::*;
use gdnative::api::object::ConnectFlags;

#[derive(NativeClass, Default)]
#[inherit(Node2D)]
pub struct World {
    score: i32
}

#[methods]
impl World {

    fn new(_owner: &Node2D) -> Self {
        Default::default()
    }

    #[export]
    fn _ready(&mut self, owner: TRef<Node2D>) {
        let emitter = &mut owner.get_node("./ObstacleSpawner").unwrap();
        let emitter = unsafe { emitter.assume_safe() };
        emitter.connect("obstacle_spawned",
                        owner,
                        "on_obstacle_created",
                        Default::default(),
                        ConnectFlags::DEFERRED.into()
        ).unwrap();
        // let subscriber = unsafe { owner.get_node_as::<CanvasLayer>("Hud").unwrap() };
        /* emitter.connect("score", subscriber, "update_score", VariantArray::new_shared(), 0)
               .unwrap();
        */
    }

    #[export]
    fn update_score(&mut self, _owner: &Node2D, score: i32) {
        godot_print!("Updating Score to: {}", score);
    }

    #[export]
    fn on_obstacle_created(&mut self, _owner: &Node2D, _obstacle: Variant) {
        // need to downcast `Variant` to `Obstacle` type.
        godot_print!("CREATED");
    }
}
