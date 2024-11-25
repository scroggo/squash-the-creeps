use std::f64::consts::PI;

use godot::classes::{
    AnimationPlayer, CharacterBody3D, ICharacterBody3D, VisibleOnScreenNotifier3D,
};
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
    fn squashed(consecutive_bounces: i32);

    // Helper methods to provide a straight line of mobs for testing
    fn choose_angle(straight_line: bool) -> f32 {
        const PI_OVER_FOUR: f64 = PI * 0.25;
        match straight_line {
            true => 0.0,
            false => randf_range(-PI_OVER_FOUR, PI_OVER_FOUR) as f32,
        }
    }

    fn choose_speed(&self, straight_line: bool) -> i64 {
        match straight_line {
            true => self.min_speed,
            false => randi_range(self.min_speed, self.max_speed),
        }
    }

    pub fn initialize(
        &mut self,
        start_position: Vector3,
        player_position: Vector3,
        straight_line: bool,
    ) {
        let scale = randf_range(0.5, 1.5) as f32;
        self.base_mut()
            .set_scale(Vector3::from_tuple((scale, scale, scale)));

        self.base_mut()
            .look_at_from_position(start_position, player_position);

        self.base_mut().rotate_y(Self::choose_angle(straight_line));

        let random_speed = self.choose_speed(straight_line);
        let mut velocity = Vector3::FORWARD * random_speed as f32;
        velocity = velocity.rotated(Vector3::UP, self.base().get_rotation().y);
        self.base_mut().set_velocity(velocity);

        let speed_scale = random_speed as f32 / self.min_speed as f32;
        self.base()
            .get_node_as::<AnimationPlayer>("AnimationPlayer")
            .set_speed_scale(speed_scale);
    }

    #[func]
    fn on_screen_exited(&mut self) {
        self.base_mut().queue_free();
    }

    pub fn squash(&mut self, consecutive_bounces: i32) {
        self.base_mut()
            .emit_signal("squashed", &[consecutive_bounces.to_variant()]);
        self.base_mut().queue_free();
    }
}
