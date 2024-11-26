use godot::classes::file_access::ModeFlags;
use godot::classes::{FileAccess, HBoxContainer, IHBoxContainer, Json, Label};
use godot::global::pow;
use godot::prelude::*;

const SAVED_HI_SCORE_PATH: &str = "user://hi_score.save";
const HI_SCORE_STR: &str = "hi_score";

#[derive(GodotClass)]
#[class(base=HBoxContainer)]
pub struct ScoreContainer {
    score: i32,
    hi_score: i32,

    #[export]
    /// At the beginning of the game, reset the hi score to 0.
    erase_hi_score_for_debugging: bool,

    base: Base<HBoxContainer>,
}

#[godot_api]
impl IHBoxContainer for ScoreContainer {
    fn init(base: Base<HBoxContainer>) -> Self {
        Self {
            score: 0,
            hi_score: 0,
            erase_hi_score_for_debugging: false,
            base,
        }
    }

    fn ready(&mut self) {
        if self.erase_hi_score_for_debugging {
            self.save_score(0);
        } else {
            self.load_hi_score();
        }
    }
}

#[godot_api]
impl ScoreContainer {
    #[func]
    fn on_mob_squashed(&mut self, consecutive_bounces: i32) {
        self.base()
            .get_node_as::<AudioStreamPlayer>("Squish")
            .play();
        self.score += pow(2.0, (consecutive_bounces - 1) as f64) as i32;
        if self.score > self.hi_score {
            self.hi_score = self.score;
        }
        self.update_scores();
    }

    fn update_scores(&mut self) {
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

    fn load_hi_score(&mut self) {
        let save_file: Gd<FileAccess> = match FileAccess::open(SAVED_HI_SCORE_PATH, ModeFlags::READ)
        {
            Some(file) => file,
            None => {
                godot_error!("Failed to open \"{}\"!", SAVED_HI_SCORE_PATH);
                return;
            }
        };

        let raw_string = save_file.get_line();
        let mut json_data = Json::new_gd();
        let err = json_data.parse(&raw_string);

        match err {
            godot::global::Error::OK => {
                let d = json_data.get_data().to::<Dictionary>();
                self.hi_score = d.at(HI_SCORE_STR).to::<f32>() as i32;
                self.update_scores();
            }
            e => {
                godot_print!("Failed to parse the JSON for with error: {}", e.as_str());
            }
        }
    }

    pub fn save_hi_score(&self) {
        if self.hi_score == self.score {
            // Tied or beat the hi score.
            self.save_score(self.hi_score);
        }
    }

    fn save_score(&self, hi_score: i32) {
        let mut save_file = FileAccess::open(SAVED_HI_SCORE_PATH, ModeFlags::WRITE)
            .expect("Failed to open SAVED_HI_SCORE_PATH for writing!");
        let d = dict! { HI_SCORE_STR: hi_score};
        let json_string = Json::stringify(&d.to_variant());
        save_file.store_line(&json_string);
    }
}
