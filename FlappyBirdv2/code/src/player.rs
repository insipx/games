use gdnative::prelude::*;
use gdnative::api::{RigidBody2D, AnimationPlayer};

pub const FLAP_FORCE: f32 = -256.0;
pub const MAX_ROTATION_DEGREES: f64 = -30.0;

/// the 'Player' Class
#[derive(NativeClass)]
#[inherit(RigidBody2D)]
pub struct Player {
    has_started: bool,
}

type Animator<'a> = TRef<'a, AnimationPlayer, Shared>;

#[methods]
impl Player {

    fn new(_owner: &RigidBody2D) -> Self {
        Player {
            has_started: false,
        }
    }

    #[export]
    fn _ready(&mut self, _owner: &RigidBody2D) {
        godot_print!("Sup MotherFuckers!!!");
    }

    #[export]
    fn _physics_process(&mut self, owner: &RigidBody2D, _delta: f32) {

        let animator = unsafe {
            owner
                .get_node_as::<AnimationPlayer>("AnimationPlayer")
                .expect("[FATAL] Could not find flappy animation! :(")
        };

        let input = Input::godot_singleton();
        if input.is_action_just_pressed("flap") {
            if !self.has_started {
                self.start(owner, animator);
            }
            self.flap(owner, animator);
        }
        
        if owner.rotation_degrees() <= MAX_ROTATION_DEGREES {
            owner.set_angular_velocity(0.0);
            owner.set_rotation_degrees(MAX_ROTATION_DEGREES);
        }
        if owner.linear_velocity().y > 0.0  {
            if owner.rotation_degrees() <= 90.0 {
                owner.set_angular_velocity(3.0);
            } else {
                owner.set_angular_velocity(0.0);
             }
        }

    }
    
    fn start(&mut self, owner: &RigidBody2D, animator: Animator<'_>) {
        if self.has_started { return } else { self.has_started = true; }
        owner.set_gravity_scale(5.0);
        animator.play("flap", -1.0, 1.0, false);
    }

    fn flap(&mut self, owner: &RigidBody2D, _animator: Animator<'_>) {
        owner.set_linear_velocity(Vector2::new(0.0, FLAP_FORCE));
        owner.set_angular_velocity(-8.0);
    }
}
