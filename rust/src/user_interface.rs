use godot::classes::{ColorRect, Control, IControl, InputEvent};
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
                let paused = tree.is_paused();
                tree.set_pause(!paused);
                self.state = match tree.is_paused() {
                    true => State::Paused,
                    false => State::Playing,
                }
            }
        }
    }
}

#[godot_api]
impl UserInterface {
    pub fn show_retry(&mut self) {
        self.state = State::GameOver;
        self.base().get_node_as::<ColorRect>("Shade").show();
    }
}
