use godot::classes::{ColorRect, Control, IControl, InputEvent};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Control)]
struct UserInterface {
    base: Base<Control>,
}

#[godot_api]
impl IControl for UserInterface {
    fn init(base: Base<Control>) -> Self {
        Self { base }
    }
    fn unhandled_input(&mut self, event: Gd<InputEvent>) {
        if event.is_action_pressed("ui_accept") {
            if self.base().get_node_as::<ColorRect>("Retry").is_visible() {
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
            }
        }
    }
}
