use gdnative::prelude::*;

const UP: Vector2 = Vector2::new(0.0, -1.0);
const FLAP: f32 = 175.0;
const MAX_FALL_SPEED: f32 = 300.0;
const GRAVITY: f32 = 10.0;

#[derive(NativeClass)]
#[inherit(KinematicBody2D)]
#[register_with(Self::register_player)]
pub struct Player {
    velocity: Vector2,
}

#[methods]
impl Player {
    fn register_player(builder: &ClassBuilder<Self>) {
        builder.add_signal(Signal {
            name: "moar_walls",
            args: &[],
        });
    }

    fn new(_owner: &KinematicBody2D) -> Self {
        Player {
            velocity: Vector2::new(0.0, 0.0),
        }
    }

    #[export]
    fn _ready(&mut self, _owner: &KinematicBody2D) {
        godot_print!("What's up motherfuckers!!");
    }

    #[export]
    fn _physics_process(&mut self, owner: &KinematicBody2D, _delta: f64) {
        let input = Input::godot_singleton();

        self.velocity.y += GRAVITY;
        if self.velocity.y > MAX_FALL_SPEED {
            self.velocity.y = MAX_FALL_SPEED;
        }
    
        if input.is_action_pressed("FLAP") {
            self.velocity.y = -FLAP;
        }

        owner.move_and_slide(self.velocity, UP, false, 10, 45.0, false);
    }
}
