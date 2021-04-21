use gdnative::prelude::*;
use gdnative::api::object::ConnectFlags;

#[derive(NativeClass, Default)]
#[inherit(Node2D)]
pub struct World;

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
