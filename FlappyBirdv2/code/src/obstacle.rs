use gdnative::prelude::*;
use gdnative::api::{PhysicsBody2D, Node};

const SPEED: f32 = 215.0;

#[derive(NativeClass, Default)]
#[inherit(Node2D)]
#[register_with(Self::register_signals)]
pub struct Obstacle {
    position: Vector2,
}

#[methods]
impl Obstacle {
    fn new(_owner: &Node2D) -> Self {
        Obstacle {
            position: Vector2::default(),
        }
    }

    fn register_signals(builder: &ClassBuilder<Self>) {
        builder.add_signal(Signal {
            name: "score",
            args: &[],
        });
    }
    
    #[export]
    fn _ready(&mut self, owner: &Node2D) {
        self.position = owner.position();
    }

    #[export]
    fn _physics_process(&mut self, owner: &Node2D, delta: f32) {
        self.position.x += -SPEED * delta;
        owner.set_position(self.position);
        if owner.global_position().x <= -200.0 {
            owner.queue_free()
        }
    }

    #[export]
    fn _on_wall_body_entered(&self, _owner: &Node2D, body: Ref<PhysicsBody2D>) {
        let collision = unsafe {
            TRef::upcast::<Node>(&body.assume_safe())
        };

        if collision.name() == "Player".into() {
            godot_print!("We should head out");
        }
    }

    #[export]
    fn _on_score_area_body_exited(&self, owner: &Node2D, body: Ref<PhysicsBody2D>) {
        let collision = unsafe {
            TRef::upcast::<Node>(&body.assume_safe())
        };
        if collision.name() == "Player".into() {
            owner.emit_signal("score", Default::default());
        }
    }
}
