use gdnative::prelude::*;
use gdnative::api::object::ConnectFlags;
use crate::obstacle::Obstacle;

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
    fn update_score(&mut self, owner: &Node2D) {
        self.score  += 1;
        godot_print!("Updating Score to: {}", self.score);
    }

    #[export]
    fn on_obstacle_created(&mut self, owner: TRef<Node2D>, obstacle: Variant) {
        let obstacle = Ref::<Node2D, Shared>::from_variant(&obstacle).unwrap();
        let obstacle = unsafe { obstacle.assume_safe() };

        let hud = &mut owner.get_node("./Hud").unwrap();
        let hud = unsafe { hud.assume_safe() };
        obstacle.connect("score",
                         hud,
                         "update_score",
                         Default::default(),
                         ConnectFlags::DEFERRED.into()
        ).unwrap();
    }
}
