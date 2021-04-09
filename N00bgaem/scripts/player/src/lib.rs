use gdnative::prelude::*;

const SPEED: f32 = 6.0;

#[derive(NativeClass)]
#[inherit(KinematicBody)]
struct Player {
    velocity: Vector3,
}

#[methods]
impl Player {
    fn new(_owner: &KinematicBody) -> Self {
        Player {
            velocity: Vector3::new(0.0, 0.0, 0.0)
        }
    }
    
    #[export]
    fn _ready(&mut self, mut _owner: &KinematicBody) {
        godot_print!("What's up, Motherfuckers!!!");
        // owner.set_physics_process(true);
    }
   
    // this runs at constant 60fps
    #[export]
    fn _physics_process(&mut self, owner: &KinematicBody, _delta: f64)  {
        let input = Input::godot_singleton();

        if input.is_action_pressed("ui_right") && input.is_action_pressed("ui_left") {
            self.velocity.x = 0.0;
        } else if input.is_action_pressed("ui_right") {
            self.velocity.x = SPEED;
        } else if input.is_action_pressed("ui_left") {
            self.velocity.x = -SPEED;
        } else {
            self.velocity = self.velocity.lerp(Vector3::new(0.0, 0.0, self.velocity.z), 0.10);
        }

        if input.is_action_pressed("ui_up") && input.is_action_pressed("ui_down") {
            self.velocity.z = 0.0;
        } else if input.is_action_pressed("ui_up") {
            self.velocity.z = -SPEED;
        } else if input.is_action_pressed("ui_down") {
            self.velocity.z = SPEED;
        } else {
            self.velocity = self.velocity.lerp(Vector3::new(self.velocity.x, 0.0, 0.0), 0.1);
        }
        owner.move_and_slide(self.velocity, Vector3::default(), true, 10, 45.0, true);
    }
}

fn init(handle: InitHandle) {
    handle.add_class::<Player>();
}

// only need this once in entire lib
godot_init!(init);
