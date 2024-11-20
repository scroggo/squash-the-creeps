use godot::classes::{ColorRect, Control, IControl, InputEvent, Label};
use godot::prelude::*;

#[derive(PartialEq)]
enum State {
    Playing,
    Paused,
    GameOver,
}

#[derive(GodotClass)]
#[class(base=Control)]
pub struct UserInterface {
    state: State,
    base: Base<Control>,
}

#[godot_api]
impl IControl for UserInterface {
    fn init(base: Base<Control>) -> Self {
        Self {
            state: State::Playing, // TODO: Title screen?
            base,
        }
    }
    fn unhandled_input(&mut self, event: Gd<InputEvent>) {
        if event.is_action_pressed("ui_accept") {
            if self.state == State::GameOver {
                if let Some(mut tree) = self.base().get_tree() {
                    tree.reload_current_scene();
                    return;
                }
            }
        }
        if event.is_action_pressed("pause") {
            if let Some(mut tree) = self.base().get_tree() {
                let mut paused = tree.is_paused();
                tree.set_pause(!paused);
                paused = !paused;
                self.state = match paused {
                    true => {
                        self.base()
                            .get_node_as::<Label>("Shade/Label")
                            .set_text("P A U S E D");
                        self.base().get_node_as::<ColorRect>("Shade").show();
                        State::Paused
                    }
                    false => {
                        self.base().get_node_as::<ColorRect>("Shade").hide();
                        State::Playing
                    }
                };
                self.base_mut().emit_signal("paused", &[paused.to_variant()]);
            }
        }
    }
}

#[godot_api]
impl UserInterface {
    #[signal]
    fn paused(paused: bool);

    pub fn show_retry(&mut self) {
        self.state = State::GameOver;
        self.base()
            .get_node_as::<Label>("Shade/Label")
            .set_text("Retry?");
        self.base().get_node_as::<ColorRect>("Shade").show();
    }
}
