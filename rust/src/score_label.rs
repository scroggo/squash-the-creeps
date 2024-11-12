use godot::classes::{ILabel, Label};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Label)]
pub struct ScoreLabel {
    score: i32,
    base: Base<Label>,
}

#[godot_api]
impl ILabel for ScoreLabel {
    fn init(base: Base<Label>) -> Self {
        Self { score: 0, base }
    }
}

#[godot_api]
impl ScoreLabel {
    #[func]
    fn on_mob_squashed(&mut self) {
        self.score += 1;
        let s = format!("Score: {}", self.score);
        // TODO: Figure out a clearer way to convert this `String`?
        self.base_mut()
            .set_text(&<String as Into<GString>>::into(s));
    }
}
