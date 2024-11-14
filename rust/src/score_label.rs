use godot::classes::{HBoxContainer, IHBoxContainer, Label};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=HBoxContainer)]
pub struct ScoreContainer {
    score: i32,
    hi_score: i32,
    base: Base<HBoxContainer>,
}

#[godot_api]
impl IHBoxContainer for ScoreContainer {
    fn init(base: Base<HBoxContainer>) -> Self {
        Self {
            score: 0,
            hi_score: 0, // TODO: Read from save file
            base,
        }
    }
}

#[godot_api]
impl ScoreContainer {
    #[func]
    fn on_mob_squashed(&mut self) {
        self.score += 1;
        if self.score > self.hi_score {
            self.hi_score = self.score;
        }
        let mut s = format!("      Score: {}", self.score);
        // TODO: Figure out a clearer way to convert this `String`?
        self.base_mut()
            .get_node_as::<Label>("ScoreLabel")
            .set_text(&<String as Into<GString>>::into(s));

        s = format!("Hi Score: {}      ", self.hi_score);
        self.base_mut()
            .get_node_as::<Label>("HiScoreLabel")
            .set_text(&<String as Into<GString>>::into(s));
    }
}
