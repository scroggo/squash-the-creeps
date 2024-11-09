use godot::classes::{CharacterBody3D, ICharacterBody3D};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=CharacterBody3D)]
struct Player {
    #[export]
    speed: i32, // Speed in m/s
    #[export]
    fall_acceleration: i32, // Acceleration in m/s^2
    target_velocity: Vector3,

    base: Base<CharacterBody3D>,
}

#[godot_api]
impl ICharacterBody3D for Player {
    fn init(base: Base<CharacterBody3D>) -> Self {
        Self {
            speed: 14,
            fall_acceleration: 75,
            target_velocity: Vector3::ZERO,
            base,
        }
    }

    fn physics_process(&mut self, delta: f64) {
        let mut direction = Vector3::ZERO;

        let input = Input::singleton();

        direction.x = input.get_axis("move_left", "move_right");
        direction.z = input.get_axis("move_forward", "move_back");

        if direction != Vector3::ZERO {
            direction = direction.limit_length(Some(1.0));
            let mut pivot = self.base().get_node_as::<Node3D>("Pivot");
            pivot.set_basis(Basis::new_looking_at(direction, Vector3::UP, false));
        }

        self.target_velocity.x = direction.x * self.speed as f32;
        self.target_velocity.z = direction.z * self.speed as f32;

        // Note to self: Not in the tutorial (yet), but shouldn't y velocity
        // reset at some point? (Maybe handled by jumping?)
        if !self.base().is_on_floor() {
            self.target_velocity.y -= delta as f32 * self.fall_acceleration as f32;
        }

        let target_velocity = self.target_velocity;
        self.base_mut().set_velocity(target_velocity);
        self.base_mut().move_and_slide();
    }
}
