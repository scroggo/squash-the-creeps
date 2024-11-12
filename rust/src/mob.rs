use std::f64::consts::PI;

use godot::classes::{CharacterBody3D, ICharacterBody3D, VisibleOnScreenNotifier3D};
use godot::global::{randf_range, randi_range};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=CharacterBody3D)]
pub struct Mob {
    #[export]
    min_speed: i64,

    #[export]
    max_speed: i64,

    base: Base<CharacterBody3D>,
}

#[godot_api]
impl ICharacterBody3D for Mob {
    fn init(base: Base<CharacterBody3D>) -> Self {
        Self {
            min_speed: 10,
            max_speed: 18,
            base,
        }
    }

    fn ready(&mut self) {
        let callable = self.base().callable("on_screen_exited");
        self.base()
            .get_node_as::<VisibleOnScreenNotifier3D>("VisibleOnScreenNotifier3D")
            .connect("screen_exited", &callable);
    }

    fn physics_process(&mut self, _delta: f64) {
        self.base_mut().move_and_slide();
    }
}

#[godot_api]
impl Mob {
    #[signal]
    fn squashed();

    pub fn initialize(&mut self, start_position: Vector3, player_position: Vector3) {
        // Note: I'm guessing this defaults to using UP as up. If it doesn't,
        // I can use the `ex` version to specify up.
        self.base_mut()
            .look_at_from_position(start_position, player_position);

        const PI_OVER_FOUR: f64 = PI * 0.25;
        self.base_mut()
            .rotate_y(randf_range(-PI_OVER_FOUR, PI_OVER_FOUR) as f32);

        let random_speed = randi_range(self.min_speed, self.max_speed);
        let mut velocity = Vector3::FORWARD * random_speed as f32;
        velocity = velocity.rotated(Vector3::UP, self.base().get_rotation().y);
        self.base_mut().set_velocity(velocity);
    }

    #[func]
    fn on_screen_exited(&mut self) {
        self.base_mut().queue_free();
    }

    pub fn squash(&mut self) {
        self.base_mut().emit_signal("squashed", &[]);
        self.base_mut().queue_free();
    }
}
