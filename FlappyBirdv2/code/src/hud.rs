use gdnative::prelude::*;

#[derive(NativeClass, Default)]
#[inherit(CanvasLayer)]
pub struct Hud {
    score: i32,
}

#[methods]
impl Hud {
    fn new(_owner: &CanvasLayer) -> Self {
        Default::default()
    }

    #[export]
    fn _ready(&mut self, owner: &CanvasLayer) {
        let score_label = unsafe {
            owner.get_node_as::<Label>("Score").expect("could not get node as label")
        };
        score_label.set_text("fuck".to_string());
    }

    #[export]
    fn update_score(&mut self, owner: &CanvasLayer) {
        godot_print!("UPDATING SCORE");
        self.score += 1;
        godot_print!("here1");
        let score_label = unsafe {
            owner.get_node_as::<Label>("Score").expect("Could not get node as label")
        };
        score_label.set_text(self.score.to_string())
    }
}
