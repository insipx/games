use gdnative::prelude::*;
use gdnative::api::{Timer, ResourceLoader, PackedScene};

#[derive(NativeClass)]
#[inherit(Node2D)]
#[register_with(Self::register_signals)]
pub struct ObstacleSpawner {
    obstacle: Option<Obstacle>
}

type Obstacle = Ref<PackedScene, Shared>;

#[methods]
impl ObstacleSpawner {
    fn new(_owner: &Node2D) -> Self {
        Self { obstacle: None }
    }

    fn register_signals(builder: &ClassBuilder<Self>) {
        builder.add_signal(Signal {
            name: "obstacle_spawned",
            args: &[]
        });
    }
    
    #[export]
    fn _ready(&mut self, _owner: &Node2D) {
        godot_print!("_ready");
        let obs = ResourceLoader::godot_singleton()
            .load("res://environment/Obstacle.tscn", "PackedScene", false)
            .expect("[FATAL] could not load resource `Obstacle`");
        self.obstacle.replace(obs.cast::<PackedScene>()
            .expect("[FATAL] Could not load obstacle scene"));
    }

    #[export]
    fn _on_timer_timeout(&self, owner: &Node2D) {
        self.spawn_obstacle(owner);
    }

    fn spawn_obstacle(&self, owner: &Node2D) {
        let obstacle = unsafe {
            self.obstacle
                .as_ref()
                .unwrap()
                .assume_safe()
                .instance(PackedScene::GEN_EDIT_STATE_DISABLED)
        };

        if let Some(o) = obstacle {
            let node = unsafe { o.assume_safe().cast::<Node2D>().unwrap() };
            let pos = (rand::random::<u16>() % 355) + 170;
            node.set_position(Vector2::new(Default::default(), pos as f32));
            owner.add_child(o, false);
            owner.emit_signal("obstacle_spawned", &[o.owned_to_variant()]);
        } else {
            godot_print!("[ERROR] no obstacle for instance 'Obstacle'");
        }
    }

    #[export]
    fn start(&self, owner: &Node2D) {
        let timer = unsafe {
            owner.get_node_as::<Timer>("SpawnTimer").unwrap()
        };
        timer.start(Default::default())
    }

    #[export]
    fn stop(&self, owner: &Node2D) {
        let timer = unsafe {
            owner.get_node_as::<Timer>("SpawnTimer").unwrap()
        };
        timer.stop()
    }
}
